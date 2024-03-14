#![deny(unsafe_code, warnings)]
use libftd2xx::{list_devices, FtStatus, Ftdi, FtdiCommon};

fn read_chip_id<T: FtdiCommon>(device: &mut T) -> Result<u32, FtStatus> {
    // https://vyvoj.hw.cz/navrh-obvodu/je-skutecne-ftdichip-id-v-ft232r-unikatni.html
    let w1 = device.eeprom_word_read(0x43)? as u32;
    let w2 = device.eeprom_word_read(0x44)? as u32;

    println!("w1 {:#04x}", w1);
    println!("w2 {:#04x}", w2);

    let chip_id = w1 | (w2 << 16);

    println!("ch {:#08x}", chip_id);

    fn bit_shuffling(b_sn: u32) -> u32 {
        (((((((b_sn & 0x2) << 0x1) | (b_sn & 0xF8)) << 0x3) | (b_sn & 0x1)) << 1)
            | ((((((b_sn >> 2) & 0x10) | (b_sn & 0x87)) >> 0x1) | (b_sn & 0x30)) >> 1))
            & 0xff
    }

    let chip_id: u32 = ((bit_shuffling(chip_id & 0xff) << 24)
        | (bit_shuffling((chip_id >> 8) & 0xff) << 16)
        | (bit_shuffling((chip_id >> 16) & 0xff) << 8)
        | bit_shuffling((chip_id >> 24) & 0xff))
        ^ 0xA5F0F7D1;

    Ok(chip_id)
}

fn main() -> Result<(), FtStatus> {
    let mut devices = list_devices()?;

    while let Some(device) = devices.pop() {
        println!("device: {device:?}");
    }

    let mut ft = Ftdi::new()?;
    let dev_type = ft.device_type()?;
    // let chip_id = ft.chip_id()?;
    let chip_id = read_chip_id(&mut ft)?;
    println!("Device type: {:?}", dev_type);
    println!("Chip id: {:#08x}", chip_id);

    Ok(())
}
