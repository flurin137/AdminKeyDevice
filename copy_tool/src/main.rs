mod clipboard;
mod device;
mod serial_wrapper;
mod validation;

use crate::clipboard::ClipboardTextReader;
use crate::device::AdminKey;

fn main() -> Result<(), String> {
    let device = AdminKey::connect()?;
    let mut clipboard = ClipboardTextReader::new();

    let data = clipboard.read()?;

    device.write(&data)
}
