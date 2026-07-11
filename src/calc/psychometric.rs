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
    pub saturation_vapor_pressure: f32
                       /* Hectopascals (hPa) */
    }

pub mod calculator {
use crate::calc::psychometric::SensorData;
use crate::calc::psychometric::PsychometricData;
use crate::calc::saturation_vapor_pressure::calculate_saturation_vapor_pressure;

    /* calculate data from initial sensor readings */
    impl SensorData {
        pub fn calculate( &self) ->Option<PsychometricData>{

            /* unwrap values from sensor */
            let temp = self.temperature?;
            let pres = self.pressure?;
            let hum = self.humidity?;

            /* collect data */
            let svp = calculate_saturation_vapor_pressure(temp);

            Some(PsychometricData {
                temperature: temp,
                pressure: pres,
                humidity: hum,
                saturation_vapor_pressure: svp
                })
            }
        }

    }