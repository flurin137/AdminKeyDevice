use defmt::warn;
use embassy_rp::{
    flash::{self, Async, ERASE_SIZE},
    peripherals::FLASH,
};

pub struct FlashWrapper<'a> {
    flash: embassy_rp::flash::Flash<'a, FLASH, Async, FLASH_SIZE>,
}

const FLASH_SIZE: usize = 2 * 1024 * 1024;
const ADDR_OFFSET: u32 = 0x100000;

pub fn print_error(error: flash::Error) {
    match error {
        embassy_rp::flash::Error::OutOfBounds => warn!("OutOfBounds"),
        embassy_rp::flash::Error::Unaligned => warn!("Unaligned"),
        embassy_rp::flash::Error::InvalidCore => warn!("InvalidCore"),
        embassy_rp::flash::Error::Other => warn!("Other"),
    }
}

impl<'a> FlashWrapper<'a> {
    pub fn new(flash: embassy_rp::flash::Flash<'a, FLASH, Async, FLASH_SIZE>) -> Self {
        Self { flash }
    }

    pub fn write_bytes(&mut self, data: &[u32; 64]) -> Result<(), flash::Error> {
        let mut read_buf = [0u8; ERASE_SIZE];

        self.flash.blocking_read(ADDR_OFFSET, &mut read_buf)?;

        self.flash
            .blocking_erase(ADDR_OFFSET, ADDR_OFFSET + ERASE_SIZE as u32)?;

        self.flash.blocking_read(ADDR_OFFSET, &mut read_buf)?;

        if read_buf.iter().any(|x| *x != 0xFF) {
            defmt::panic!("unexpected");
        }

        for (i, entry) in data.iter().enumerate() {
            let index = i as u32;
            self.flash
                .blocking_write(ADDR_OFFSET + index, &entry.to_ne_bytes())?;
        }

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), flash::Error> {
        let mut buf = [0u8; ERASE_SIZE];
        self.flash.blocking_read(ADDR_OFFSET, &mut buf)?;

        self.flash
            .blocking_erase(ADDR_OFFSET, ADDR_OFFSET + ERASE_SIZE as u32)?;

        self.flash.blocking_read(ADDR_OFFSET, &mut buf)?;
        if buf.iter().any(|x| *x != 0xFF) {
            defmt::panic!("unexpected");
        }

        for b in buf.iter_mut() {
            *b = 0xDA;
        }

        self.flash.blocking_write(ADDR_OFFSET, &buf)?;
        self.flash.blocking_read(ADDR_OFFSET, &mut buf)
    }

    pub async fn read(&mut self) -> Result<[u32; 64], flash::Error> {
        let mut buf = [0u32; 64];
        self.flash.background_read(ADDR_OFFSET, &mut buf)?.await;
        Ok(buf)
    }
}
