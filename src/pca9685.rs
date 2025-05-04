use cortex_m_semihosting::hprintln;
use stm32f4xx_hal::{i2c::{Address, Error, I2c}, pac::I2C1, time::{Hertz, KiloHertz}};
use bitfield_struct::bitfield;

pub struct PCA9685 {
    i2c: I2c<I2C1>,
    address: Address,
}

#[bitfield(u8)]
pub struct Mode1 {
    pub allcall: bool,   // Bit 0
    pub sub3: bool,      // Bit 1
    pub sub2: bool,      // Bit 2
    pub sub1: bool,      // Bit 3
    pub sleep: bool,     // Bit 4
    pub ai: bool,        // Bit 5 (Auto-Increment)
    pub extclk: bool,    // Bit 6
    pub restart: bool,   // Bit 7
}

#[bitfield(u8)]
pub struct Mode2 {
    #[bits(2)]
    pub outne: u8,
    pub outdrv: bool,
    pub och: bool,
    pub invrt: bool,
    #[bits(3)]
    __: u8
}


impl PCA9685 {
    pub fn new(mut i2c: I2c<I2C1>) -> Self {
        let address = Address::Seven(0x40);

        let mut mode = [0];
        let _ = i2c.write_read(address, &[0x0], &mut mode);


        let mut mode1: Mode1 = mode[0].into();
        mode1.set_ai(true);

        let _ = i2c.write(address, &[0, mode1.into()]);

        let mut mode2 = Mode2::new();
        mode2.set_outdrv(true); // totem pole config

        

        let _ = i2c.write(address, &[0x1, mode2.into()]);
        Self { i2c, address }

    }
    pub fn set_pwm_frequency(&mut self, freq: Hertz) -> Result<(), Error>{
        let prescalar = (25_000_000f32/(4096f32*freq.raw() as f32)) + 0.5 - 1f32;

        let mut mode = [0];
        self.i2c.write_read(self.address, &[0], &mut mode)?;

        let mut mode1: Mode1 = mode[0].into();
        mode1.set_sleep(true);
        mode1.set_restart(false);

        self.i2c.write(self.address, &[0x0, mode1.into()])?;
        self.i2c.write(self.address, &[0xFE, prescalar as u8])?;
        
        mode1.set_sleep(false);
        mode1.set_restart(true);

        self.i2c.write(self.address, &[0x0, mode1.into()])?;

        Ok(())
    }

    pub fn set_duty_cycle(&mut self, ch: u8, duty: f32) -> Result<(), Error> {
        let on = 0u16;
        let off = (duty * 4096.0) as u16;

        let base = 0x6 + (ch * 4);

        let on_l = (on & 0xFF) as u8;
        let on_h = (on >> 8) as u8;
        let off_l = (off & 0xFF) as u8;
        let off_h = (off >> 8) as u8;

        
        self.i2c.write(self.address, &[base, on_l, on_h, off_l, off_h])?;


        Ok(())

    }
}