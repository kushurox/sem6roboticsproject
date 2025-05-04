#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use pca9685::PCA9685;
use stm32f4xx_hal::{gpio::{Alternate, Pin}, i2c, pac, prelude::*, rcc::RccExt};


mod pca9685;



#[entry]
fn kmain() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
    .use_hse(25.MHz())
    .sysclk(48.MHz())
    .pclk1(24.MHz())
    .require_pll48clk()
    .freeze();

    let gpiob = dp.GPIOB.split();

    let scl: Pin<'B', 8, Alternate<4, _>> = gpiob.pb8.into_alternate().set_open_drain();
    let sda: Pin<'B', 9, Alternate<4, _>> = gpiob.pb9.into_alternate().set_open_drain();

    let i2c = i2c::I2c::new(dp.I2C1, (scl, sda), i2c::Mode::standard(100.kHz()), &clocks);
    let mut pca = PCA9685::new(i2c);

    let _ = pca.set_pwm_frequency(50.Hz());
    let _ = pca.set_duty_cycle(0, 0.3);
    let _ = pca.set_duty_cycle(1, 0.5);


    loop{};
}