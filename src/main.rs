#![no_main]
#![no_std]

use cortex_m::{delay::Delay, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use pca9685::PCA9685;
use static_cell::StaticCell;
use stm32f4xx_hal::{gpio::{Alternate, Pin}, i2c, pac, prelude::*, rcc::RccExt};

use stm32f4xx_hal::otg_fs::{USB, UsbBus};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};


mod pca9685;

static EP_MEMORY: StaticCell<[u32; 1024]> = StaticCell::new();

#[entry]
fn kmain() -> ! {

    let ep_mem = unsafe { EP_MEMORY.init([0; 1024]) };
    let dp = pac::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let syst = cp.SYST;
    let mut d = Delay::new(syst, 48_000_000);

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
    pca.set_duty_cycle(4, degrees_to_ds(0f32));
    d.delay_ms(3000);
    pca.set_duty_cycle(4, degrees_to_ds(40f32));
    // let mut ds = 0f32;
    // let mut inc = 0.01;

    // loop {
    //     ds += inc;
    //     if ds >= 0.2 {inc = -0.01}
    //     else if ds <= 0.0{
    //         inc = 0.01
    //     }
    //     let _ = pca.set_duty_cycle(4, ds);
    //     let _ = pca.set_duty_cycle(0, ds);
    //     d.delay_ms(100);

    // }

    // let gpioa = dp.GPIOA.split();
    // let usb_dm = gpioa.pa11.into_alternate();
    // let usb_dp = gpioa.pa12.into_alternate();

    // // Set up USB peripheral
    // let usb = USB {
    //     usb_global: dp.OTG_FS_GLOBAL,
    //     usb_device: dp.OTG_FS_DEVICE,
    //     usb_pwrclk: dp.OTG_FS_PWRCLK,
    //     pin_dm: usb_dm.into(),
    //     pin_dp: usb_dp.into(),
    //     hclk: clocks.hclk()
    // };

    // // Create USB bus and serial port
    // let usb_bus = UsbBus::new(usb, ep_mem);
    // let mut serial = SerialPort::new(&usb_bus);
    // let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    // .strings(&[StringDescriptors::default()
    // .manufacturer("kushurox")
    // .product("totm")
    // .serial_number("TEST")
    // ]).unwrap()
    // .device_class(USB_CLASS_CDC).build();

    loop{
        // if usb_dev.poll(&mut [&mut serial]) {
        //     let mut buf = [0u8; 64];
        //     if let Ok(count) = serial.read(&mut buf) {
        //         hprintln!("count: {}", count);
        //         if count < 16 {continue;}
        //         let vals: PWMVals = buf.into();
        //         hprintln!("duties: {} {} {} {}", vals.duty1, vals.duty2, vals.duty3, vals.duty4);   //  remove this when working with the real thing
        //         let _ = pca.set_duty_cycle(0, vals.duty1);
        //         let _ = pca.set_duty_cycle(1, vals.duty2);      // father forgive me, for I have sinned to count from 1
        //         let _ = pca.set_duty_cycle(2, vals.duty3);
        //         let _ = pca.set_duty_cycle(3, vals.duty4);
        //     }
        // }
    };
}

struct PWMVals {
    duty1: f32,
    duty2: f32,
    duty3: f32,
    duty4: f32
}

impl From<[u8; 64]> for PWMVals {
    fn from(value: [u8; 64]) -> Self {
        let db1: [u8; 4] = value[0..4].try_into().unwrap();
        let db2: [u8; 4] = value[4..8].try_into().unwrap();
        let db3: [u8; 4] = value[8..12].try_into().unwrap();
        let db4: [u8; 4] = value[12..16].try_into().unwrap();

        Self {
            duty1: f32::from_le_bytes(db1),
            duty2: f32::from_le_bytes(db2),
            duty3: f32::from_le_bytes(db3),
            duty4: f32::from_le_bytes(db4)
        }
    }
}

const fn degrees_to_ds(degrees: f32) -> f32{
    // 0.2 : 180
    // 1: 0.2/180
    (0.2/180f32) * degrees
}