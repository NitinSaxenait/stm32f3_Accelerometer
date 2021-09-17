#![no_main]
#![no_std]

use cortex_m::asm;
use stm32f3_Accelerometer::config::initialization::*;
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
    let (mut lsm303dlhc, delay, mut itm) = initialization();
    match lsm303dlhc.set_accel_sensitivity(Sensitivity::G12) {
        Ok(senstivity) => senstivity,
        Err(err) => panic!("Can't set accelerometer senstivity {:?}", err),
    }
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
}
