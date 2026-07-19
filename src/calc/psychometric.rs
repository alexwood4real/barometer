/**************************************************************
 * SPDX-License-Identifier: MIT OR Apache-2.0
 * Barometer
 *
 * FILE:
 * psychometric.rs
 *
 * Description:
 * Main driver in order to calculate everything given from
 * BME 280 sensor. Gets temp, humidity, pressure, and others  
 **************************************************************/

/* holds initial sensor reading data */
pub struct SensorData {
    pub temperature: Option<f32>,  /* °C */
    pub pressure: Option<f32>,     /* Pascals */
    pub humidity: Option<f32>      /* % water vapor in air  */
    }

/* holds calculated data */
pub struct PsychometricData {
    pub temperature: f32,  /* °C */
    pub pressure: f32,     /* Pascals */
    pub humidity: f32,     /* % water vapor in air  */
    pub altitude: f32,     /* Altitude (ft) */
    pub saturation_vapor_pressure: f32,
                           /* Hectopascals (hPa) */
    pub vapor_pressure: f32,
                           /* Vapor Pressure (hPa) */
    pub dew_point: f32,    /* Dew Point °C */
    pub vapor_pressure_deficit: f32,
                           /* Vapor Pressure Deficit (hpa) */
    pub absolute_humidity: f32,
                           /* Absolute Humidity (g/m^3) */
    pub mixing_ratio: f32, /* Mixing Ratio (g water / kg dry air) */
    pub specific_humidity: f32,
                           /* Specific Humidity (kg / kg) */
    pub air_density: f32,  /* Air Density (kg / m^3) */
    pub enthalpy: f32,     /* Enthalpy (kJ / kg dry air) */
    pub wet_bulb: f32,     /* Wet Bulb (°C) */
    pub heat_index: f32,   /* Heat Index (°F) */
    }

pub mod calculator {
use crate::calc::psychometric::SensorData;
use crate::calc::psychometric::PsychometricData;
use crate::calc::absolute_humidity::calculate_absolute_humidity;
use crate::calc::air_denisty::calculate_air_density;
use crate::calc::altitude::calculate_altitude;
use crate::calc::dew_point::calculate_dew_point;
use crate::calc::enthalpy::calculate_enthalpy;
use crate::calc::heat_index::calculate_heat_index;
use crate::calc::mixing_ratio::calculate_mixing_ratio;
use crate::calc::saturation_vapor_pressure::calculate_saturation_vapor_pressure;
use crate::calc::vapor_pressure_deficit::calculate_vapor_pressure_deficit;
use crate::calc::specific_humidity:: calculate_specific_humidity;
use crate::calc::vapor_pressure::calculate_vapor_pressure;
use crate::calc::wet_bulb::calculate_wet_bulb;

    /* calculate data from initial sensor readings */
    impl SensorData {
        pub fn calculate( &self) ->Option<PsychometricData>{

            /* unwrap values from sensor */
            let temp = self.temperature?;
            let pres = self.pressure?;
            let hum = self.humidity?;

            /* collect data */
            let alt = calculate_altitude(pres);
            let svp = calculate_saturation_vapor_pressure(temp);
            let vp = calculate_vapor_pressure(svp, hum);
            let dp = calculate_dew_point(vp);
            let vpd = calculate_vapor_pressure_deficit(svp, vp);
            let ah = calculate_absolute_humidity(temp, vp);
            let mr= calculate_mixing_ratio(pres, vp);
            let sh = calculate_specific_humidity(mr);
            let ad = calculate_air_density(pres, temp, vp);
            let ent = calculate_enthalpy(temp, mr);
            let wb = calculate_wet_bulb(hum, temp);
            let hi = calculate_heat_index(temp, hum);

            Some(PsychometricData {
                temperature: temp,
                pressure: pres,
                humidity: hum,
                altitude: alt,
                saturation_vapor_pressure: svp,
                vapor_pressure: vp,
                dew_point: dp,
                vapor_pressure_deficit: vpd,
                absolute_humidity: ah,
                mixing_ratio: mr,
                specific_humidity: sh,
                air_density: ad,
                enthalpy: ent,
                wet_bulb: wb,
                heat_index: hi,
                })
            }
        }

    }