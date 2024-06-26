mod clipboard;
mod device;
mod notification_writer;
mod sanitizer;
mod serial_wrapper;
mod validation;

use notification_writer::NotificationWriter;
use sanitizer::SwissGermanLanguageMapper;
use validation::Validator;

use crate::clipboard::ClipboardTextReader;
use crate::device::AdminKey;

fn main() -> Result<(), String> {
    let mut application = Application::build()?;

    application.run()
}

struct Application {
    validator: Validator,
    clipboard: ClipboardTextReader,
    notification_writer: NotificationWriter,
}

impl Application {
    pub fn build() -> Result<Self, String> {
        let clipboard = ClipboardTextReader::new();
        let validator = Validator::new(SwissGermanLanguageMapper::new_boxed());
        let notification_writer = NotificationWriter::new("Admin Key Copy Tool".to_owned());

        Ok(Self {
            validator,
            clipboard,
            notification_writer,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        let result = self.handle_request();
        self.notification_writer.write_notification(result)
    }

    fn handle_request(&mut self) -> Result<String, String> {
        let admin_key = AdminKey::connect()?;

        let data = self.clipboard.read()?;

        let data = self.validator.validate(data)?;
        let data = self.validator.sanitize(data);

        admin_key
            .write(&data)
            .map(|_| "Written To Admin Key".to_owned())
    }
}
