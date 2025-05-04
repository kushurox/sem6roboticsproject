#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

use static_cell::StaticCell;
use stm32f4xx_hal::{pac, prelude::*, otg_fs::{USB, UsbBus}};
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

static EP_MEMORY: StaticCell<[u32; 1024]> = StaticCell::new();

#[entry]
fn main() -> ! {

    let ep_mem = unsafe { EP_MEMORY.init([0; 1024]) };
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr
    .use_hse(25.MHz())
    .sysclk(48.MHz())
    .pclk1(24.MHz())
    .require_pll48clk()
    .freeze();

    // Acquire GPIOA for USB pins
    let gpioa = dp.GPIOA.split();
    let usb_dm = gpioa.pa11.into_alternate();
    let usb_dp = gpioa.pa12.into_alternate();

    // Set up USB peripheral
    let usb = USB {
        usb_global: dp.OTG_FS_GLOBAL,
        usb_device: dp.OTG_FS_DEVICE,
        usb_pwrclk: dp.OTG_FS_PWRCLK,
        pin_dm: usb_dm.into(),
        pin_dp: usb_dp.into(),
        hclk: clocks.hclk()
    };

    // Create USB bus and serial port
    let usb_bus = UsbBus::new(usb, ep_mem);
    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    .strings(&[StringDescriptors::default()
    .manufacturer("kushurox")
    .product("totm")
    .serial_number("TEST")
    ]).unwrap()
    .device_class(USB_CLASS_CDC).build();

    // Main loop
    loop {
        if usb_dev.poll(&mut [&mut serial]) {
            let mut buf = [0u8; 64];
            if let Ok(count) = serial.read(&mut buf) {
                hprintln!("count: {}", count);
                if count < 16 {continue;}
                let vals: PWMVals = buf.into();
                hprintln!("duties: {} {} {} {}", vals.duty1, vals.duty2, vals.duty3, vals.duty4)
            }
        }
    }
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