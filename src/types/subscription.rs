use chrono::Utc;

use super::{Subscription, SubscriptionStatus};

impl Subscription {
    pub fn new(telegram_id: &str) -> Self {
        Self {
            telegram_id: telegram_id.to_string(),
            status: SubscriptionStatus::Active,
            created_at: Utc::now(),
        }
    }

    pub fn telegram_id(&self) -> &str {
        &self.telegram_id
    }
}
