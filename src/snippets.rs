    // const _PWN_FREQ: usize = 50;    // 50Hz
    // const _PRESCALAR: u8 = ((25_000_000/(4096*_PWN_FREQ)) as f32 + 0.5) as u8 - 1;  // evaluates to 0x7a
    // let target = Address::Seven(0x40);
    
    // let mut mode1 = [0];
    // let mut _res = i2c.write_read(target, &[0], &mut mode1);
    
    // let sleep_mode = (mode1[0] & !0x80) | 0x10;
    // _res = i2c.write(target, &[0x00, sleep_mode]);

    // _res = i2c.write(target, &[0xFE, _PRESCALAR]);

    // let wake_mode = (sleep_mode & !0x10) | 0x20;    // sets the AL to high as well
    // _res = i2c.write(target, &[0x00, wake_mode]);

    // _res = i2c.write(target, &[0x01, 0x04]);    // push pull non inverting, totem polem mode

    // _res = i2c.write(target, &[0x06, 0x00, 0x00, 0x00, 0x08]); // writing to channel 0  