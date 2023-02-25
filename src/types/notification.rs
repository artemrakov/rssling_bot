use super::Notification;

impl Notification {
    pub fn empty(&self) -> bool {
        self.entries.len() == 0
    }

    pub fn telegram_id(&self) -> &str {
        &self.telegram_id
    }
}
