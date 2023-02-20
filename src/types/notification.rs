use super::Notification;

impl Notification {
    pub fn empty(&self) -> bool {
        self.entries.len() == 0
    }
}
