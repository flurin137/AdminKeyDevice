use clippers::Clipboard;

pub struct ClipboardTextReader {
    clipboard: Clipboard,
}

impl ClipboardTextReader {
    pub fn new() -> Self {
        let clipboard = clippers::Clipboard::get();
        Self { clipboard }
    }

    pub fn read(&mut self) -> Result<String, String> {
        match self.clipboard.read() {
            Some(clippers::ClipperData::Text(text)) => Ok(format!("{}", text)),
            _ => Err("Invalid data in clipboard".to_owned()),
        }
    }
}
