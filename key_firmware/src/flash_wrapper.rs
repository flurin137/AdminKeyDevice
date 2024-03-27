use defmt::info;
use embassy_rp::{
    flash::{self, Blocking, Flash},
    peripherals::FLASH,
};

pub struct FlashWrapper<'a> {
    flash: embassy_rp::flash::Flash<'a, FLASH, Blocking, FLASH_SIZE>,
}

const FLASH_SIZE: usize = 2 * 1024 * 1024;
const ADDR_OFFSET: u32 = 0x100000;

pub fn print_error(error: flash::Error) {
    match error {
        embassy_rp::flash::Error::OutOfBounds => info!("OutOfBounds"),
        embassy_rp::flash::Error::Unaligned => info!("Unaligned"),
        embassy_rp::flash::Error::InvalidCore => info!("InvalidCore"),
        embassy_rp::flash::Error::Other => info!("Other"),
    }
}

impl<'a> FlashWrapper<'a> {
    pub fn new(flash: FLASH) -> Self {
        let flash = Flash::<_, Blocking, FLASH_SIZE>::new_blocking(flash);
        Self { flash }
    }

    pub fn write(&mut self, data: &[u8; 64]) -> Result<(), flash::Error> {
        self.flash.blocking_write(ADDR_OFFSET, data)
    }

    pub async fn read(&mut self) -> Result<[u8; 64], flash::Error> {
        let mut buffer = [0u8; 64];
        self.flash.blocking_read(ADDR_OFFSET, &mut buffer)?;
        Ok([0u8; 64])
    }
}
