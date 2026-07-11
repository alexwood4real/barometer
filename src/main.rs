/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * main.rs
 *
 * Description:
 * Main loop to collect data from BME 280 sensor
 * Blinking LED ensures Wi-Fi connection and data collection
 **************************************************************/

#![no_std]
#![no_main]

/* crates */
use bme280_rs::{AsyncBme280, Configuration, Oversampling, SensorMode};
use cyw43::aligned_bytes;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
//use embassy_rp::adc::Sample;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::i2c::{Config as I2cConfig, I2c};
use embassy_rp::peripherals::{DMA_CH0, PIO0, I2C0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::{Duration, Timer, Delay};
use log::info;
use static_cell::StaticCell;
use crate::calc::psychometric::{SensorData};

use {defmt_rtt as _, panic_probe as _};

/* mods */
mod calc;

/* Constants */
const WIFI_NETWORK: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

/* Interrupt Handlers */
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
    I2C0_IRQ => embassy_rp::i2c::InterruptHandler<I2C0>;
    USBCTRL_IRQ =>embassy_rp::usb::InterruptHandler<embassy_rp::peripherals::USB>;
});

/* Async functions */
#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, cyw43::SpiBus<Output<'static>, PioSpi<'static, PIO0, 0>>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn heartbeat(
    mut control: cyw43::Control<'static>
) -> ! {
    info!("Heartbeat start");
    let delay: Duration = Duration::from_millis(500);

    loop 
        {
        control.gpio_set(0, true).await;
        Timer::after(delay).await;

        control.gpio_set(0, false).await;
        Timer::after(delay).await;
        }
}

#[embassy_executor::task]
async fn net_task(
    mut runner: embassy_net::Runner<'static, cyw43::NetDriver<'static>>
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn logger_task(
    driver: embassy_rp::usb::Driver<'static, embassy_rp::peripherals::USB>
) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Barometer start");

    /* Init RP2350 peripherals */
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

    /* Initialize Logger */
    let driver = embassy_rp::usb::Driver::new(p.USB, Irqs);
    spawner.spawn(unwrap!(logger_task(driver)));

    /* Load Wi-Fi firmware */
    let fw: &cyw43::Aligned<cyw43::A4, [u8]> = aligned_bytes!("cyw43-firmware/43439A0.bin");
    let clm: &cyw43::Aligned<cyw43::A4, [u8]> = aligned_bytes!("cyw43-firmware/43439A0_clm.bin");
    let nvram: &cyw43::Aligned<cyw43::A4, [u8]> = aligned_bytes!("cyw43-firmware/nvram_rp2040.bin");

    /* Configure GPIO's and PIO/SPI */
    let pwr: Output<'_> = Output::new(p.PIN_23, Level::Low);
    let cs: Output<'_> = Output::new(p.PIN_25, Level::High);
    let mut pio: Pio<'_, PIO0> = Pio::new(p.PIO0, Irqs);
    let spi: PioSpi<'_, PIO0, 0> = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        RM2_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        dma::Channel::new(p.DMA_CH0, Irqs),
    );

    /* Allocate state driver */
    static STATE: StaticCell<cyw43::State> = StaticCell::new();

    /* Create CYW43 driver */
    let state: &mut cyw43::State = STATE.init(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;

    /* turn on background driver */
    spawner.spawn(unwrap!(cyw43_task(runner)));

    /* set up the control */
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    /* configure network stack */
    let net_config = embassy_net::Config::dhcpv4( Default::default() );
    let seed: u64 = 0x0123_4567_89ab_cdef;

    static RESOURCES: StaticCell<embassy_net::StackResources<3>> = StaticCell::new();
    let (stack, net_runner) = embassy_net::new(
        net_device,
        net_config,
        RESOURCES.init(embassy_net::StackResources::new()),
        seed
    );

    spawner.spawn(unwrap!(net_task(net_runner)));

    /* try to join the network until success */
    loop
        {
        match control
            .join(WIFI_NETWORK, cyw43::JoinOptions::new(WIFI_PASSWORD.as_bytes()))
            .await
            {
            Ok(_) => break,
            Err(err) => info!("Failed to join network, status = {:?}", err)
            }
        }

    /* Wi-Fi has been connected, waiting for DHCP */
    stack.wait_config_up().await;

    /* turn on heartbeat */
    spawner.spawn(unwrap!(heartbeat(control)));

    /* Configure BME 280 sensor */
    let i2c  = I2c::new_async(p.I2C0, p.PIN_5, p.PIN_4, Irqs, I2cConfig::default());
    let delay = Delay;
    let mut bme280= AsyncBme280::new(i2c, delay);

    unwrap!(bme280.init().await);

    unwrap!(bme280.set_sampling_configuration(
        Configuration::default()
            .with_temperature_oversampling(Oversampling::Oversample1)
            .with_pressure_oversampling(Oversampling::Oversample1)
            .with_humidity_oversampling(Oversampling::Oversample1)
            .with_sensor_mode(SensorMode::Normal)
        ).await);


    /* infinite main loop */
    loop 
        {
        /* takes all data from a single sample instead of three different ones */
        let measurements = unwrap!(bme280.read_sample().await);

        let sensor_data = SensorData {
            temperature: measurements.temperature,
            pressure: measurements.pressure,
            humidity: measurements.humidity
            };

        if let Some(weather_data) = sensor_data.calculate() {
            // next, print this out to see if it is working before doing bucks
            // also, start a branch for this
            println!("temp: {}\n
                      press: {}\n
                      hum: {}\n
                      sat: {}\n\n", 
                      weather_data.temperature,
                      weather_data.pressure,
                      weather_data.humidity,
                      weather_data.saturation_vapor_pressure);
        } else {
            info!("Uh oh");
        }
        
        
        /* wait 1 sec before going again */
        Timer::after(Duration::from_secs(1)).await;
        }
}

/* Metadata */
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"barometer"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico 2 W's onboard LED, connected to GPIO 0 of the cyw43 \
        (WiFi chip) via PIO 0 over the SPI bus."
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

// End of file
