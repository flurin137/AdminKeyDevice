use std::{io, path::Path, thread::sleep, time};

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
        let mut buffer = [0; 64];
        
        let sleep_duration = time::Duration::from_millis(5);
        sleep(sleep_duration);

        let count = self.port.read(&mut buffer)?;

        let string = String::from_utf8_lossy(&buffer[..count]).trim().to_owned();

        Ok(string)
    }
}
