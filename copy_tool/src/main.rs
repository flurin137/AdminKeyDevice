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
    let validator = Validator::new(validation::Language::DE_CH);

    let data = clipboard.read()?;

    let data = validator.validate(data)?;
    let data = validator.sanitize(data);

    device.write(&data)
}