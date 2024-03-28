mod clipboard;
mod device;
mod serial_wrapper;
mod validation;

use validation::Validator;

use crate::clipboard::ClipboardTextReader;
use crate::device::AdminKey;

fn main() -> Result<(), String> {
    let device = AdminKey::connect()?;
    let mut clipboard = ClipboardTextReader::new();
    let validator = Validator::new();

    let data = clipboard.read()?;

    let data = validator.validate(data)?;

    device.write(&data)
}
