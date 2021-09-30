#![no_main]
#![no_std]

use cortex_m::asm;
use m::Float;
use stm32f3_Accelerometer::config::initialization::*;
use stm32f3_discovery::stm32f3xx_hal::hal::blocking::delay::DelayMs;

/// This program is going to print the Accelerometer sensor readings(x, y, z-axis) on the itm-dump console with
/// 9.26 sec of delay.
///
/// #Arguments
/// None
///
/// #Return
/// This is a no_std no_main code which will neither return nor stop.
#[entry]
fn main() -> ! {
    const SENSITIVITY: f32 = 12. / (1 << 14) as f32;
    const THRESHOLD: f32 = 0.5;
    let (mut lsm303dlhc, mut delay, mut itm, mono_timer) = initialization();
    match lsm303dlhc.set_accel_sensitivity(Sensitivity::G12) {
        Ok(senstivity) => senstivity,
        Err(err) => panic!("Can't set accelerometer senstivity {:?}", err),
    }
    // Get the Acceleromter Sensor Readings....
    loop {
        const SENSITIVITY: f32 = 12. / (1 << 14) as f32;

        let I16x3 { x, y, z } = match lsm303dlhc.accel() {
            Ok(values) => values,
            Err(error) => panic!("Cannot get sensor readings {:?}", error),
        };

        let x_axis = x as f32 * SENSITIVITY;
        let y_axis = y as f32 * SENSITIVITY;
        let z_axis = z as f32 * SENSITIVITY;

        iprintln!(
            &mut itm.stim[0],
            "Accelerometer Readings \n x= {:?} y= {:?} z= {:?}",
            x_axis,
            y_axis,
            z_axis
        );

        //9.26seconds of delay--
        asm::delay(50_000_000)
    }
    /*
    // uncomment this to build the puncho-o-meter.
    // CODE FOR Punch-o-meter....
    let measurement_time = mono_timer.frequency().0; // 1 second in ticks
    let mut instant = None;
    let mut max_g = 0.;
    loop {
        let g_x = f32::from(lsm303dlhc.accel().unwrap().x).abs() * SENSITIVITY;

        match instant {
            None => {
                // If acceleration goes above a threshold, we start measuring
                if g_x > THRESHOLD {
                    iprintln!(&mut itm.stim[0], "START!");

                    max_g = g_x;
                    instant = Some(mono_timer.now());
                }
            }
            // Still measuring
            Some(ref instant) if instant.elapsed() < measurement_time => {
                if g_x > max_g {
                    max_g = g_x;
                }
            }
            _ => {
                // Report max value
                iprintln!(&mut itm.stim[0], "Max acceleration: {}g", max_g);

                // Measurement done
                instant = None;

                // Reset
                max_g = 0.;
            }
        }

        delay.delay_ms(50_u8);
    }*/
}
