/***************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * main.rs
 *
 * Description:
 * This example was pulled directly from the
 * embassy-rs repo on github
 * https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/blinky_wifi.rs
 ***************************************************/

#![no_std]
#![no_main]

/* crates */
use cyw43::aligned_bytes;
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::{bind_interrupts, dma};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

/* Handlers */
bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>;
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

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Barometer start");

    /* Init RP2350 peripherals */
    let p: embassy_rp::Peripherals = embassy_rp::init(Default::default());

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
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw, nvram).await;

    /* turn on background driver */
    spawner.spawn(unwrap!(cyw43_task(runner)));

    /* turn on heartbeat */
    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    spawner.spawn(unwrap!(heartbeat(control)));

    /* infinite main loop */
    loop 
        {
        info!("Data collection start");

        /* Data colletion, will be done with #8 */
        Timer::after(Duration::from_secs(1)).await;

        info!("Data collection end");
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
