//mod clipboard;
mod device;
mod sanitizer;
mod serial_wrapper;
mod validation;

use sanitizer::SwissGermanLanguageMapper;
use validation::Validator;

//use crate::clipboard::ClipboardTextReader;
use crate::device::AdminKey;

fn main() -> Result<(), String> {
    let device = AdminKey::connect()?;
    //let mut clipboard = ClipboardTextReader::new();
    let validator = Validator::new(SwissGermanLanguageMapper::new_boxed());

    let data = "".to_string(); // clipboard.read()?;

    let data = validator.validate(data)?;
    let data = validator.sanitize(data);

    device.write(&data)
}
