use std::{fmt::Display, io, path::PathBuf, thread::sleep, time};
use serial2::SerialPort;

pub struct SerialWrapper {
    port: SerialPort,
    path: PathBuf,
}

impl Display for SerialWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap_or("Unknown"))
    }
}

impl SerialWrapper {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let port = SerialPort::open(path.clone(), 115200)?;
        Ok(Self { port, path })
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
