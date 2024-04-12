use notify_rust::Notification;

pub struct NotificationWriter {
    application_name: String,
}

impl NotificationWriter {
    pub fn new(application_name: String) -> Self {
        Self { application_name }
    }

    pub fn write_notification(&self, result: Result<String, String>) -> Result<(), String> {
        let message = match result {
            Ok(message) => format!("Success: \n {message}"),
            Err(error) => format!("Error: \n {error}"),
        };

        Notification::new()
            .summary(&self.application_name)
            .body(&message)
            .show()
            .map_err(|e| format!("Error writing Notification {e}"))
    }
}
