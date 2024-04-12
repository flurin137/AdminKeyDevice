use notify_rust::Notification;

pub struct NotificationWriter {
    application_name: String,
}

impl NotificationWriter {
    pub fn new(application_name: String) -> Self {
        Self { application_name }
    }

    pub fn write_notification(&self, result: Result<String, String>) -> Result<(), String> {
        let (summary, message) = match result {
            Ok(message) => ("Success".to_owned(), message),
            Err(error) => ("Error".to_owned(), error),
        };

        Notification::new()
            .appname(&self.application_name)
            .summary(&summary)
            .body(&message)
            .show()
            .map_err(|e| format!("Error writing Notification {e}"))
    }
}
