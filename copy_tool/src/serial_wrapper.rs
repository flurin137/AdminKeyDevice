use std::{io, path::Path};

use serial2::SerialPort;

pub struct SerialWrapper {
    port: SerialPort,
}

impl SerialWrapper {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let port = SerialPort::open(path, 115200)?;
        Ok(Self { port })
    }

    pub fn write(&self, value: &str) -> io::Result<()> {
        let bytes = value.as_bytes();
        self.port.write(bytes)?;
        Ok(())
    }

    pub fn read(&self) -> io::Result<String> {
        let mut buffer = [0; 256];
        self.port.read(&mut buffer)?;

        Ok("".to_owned())
    }
}
