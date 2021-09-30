#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::{
    lsm303dlhc::{self, I16x3, Sensitivity},
    stm32f3xx_hal::{delay::Delay, prelude, time::MonoTimer},
};

use stm32f3_discovery::stm32f3xx_hal::{
    gpio::gpiob::{PB6, PB7},
    gpio::AF4,
    i2c::I2c,
    prelude::*,
    stm32::{self, I2C1},
};

pub type Lsm303dlhc = lsm303dlhc::Lsm303dlhc<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>;
/// This program is going to provide the initialization to get and print the sensor readings with
/// some delay.
/// #Argument
/// None
///
/// #Return
/// This program is going to return a tuple of lsm-package, delay and itm.
/// Lsm303dlhc -> a package to access the accelerometer sensor to get the readings.
/// Delay -> a time delay to pause the running program for few seconds.
/// ITM -> a tool to print the sensor data on the itm console.
pub fn initialization() -> (Lsm303dlhc, Delay, ITM, MonoTimer) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high().unwrap();

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let lsm303dlhc = Lsm303dlhc::new(i2c).expect("Cannot setup i2c in lsm303 package");

    let delay = Delay::new(cp.SYST, clocks);
    /// A clock source that only increments and never jumps - MonoTimer
    let mono_timer = MonoTimer::new(cp.DWT, clocks);

    (lsm303dlhc, delay, cp.ITM, mono_timer)
}
