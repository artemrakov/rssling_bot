use std::sync::Arc;

use super::{Channel, Notification, RssEntry, Subscription, SubscriptionStatus};

impl Channel {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn new_entries(&self) -> Vec<RssEntry> {
        self.entries
            .clone()
            .into_iter()
            .filter(|entry| entry.pub_date > self.updated_at)
            .collect()
    }

    pub fn active_subscriptions(&self) -> Vec<&Subscription> {
        self.subs
            .iter()
            .filter(|sub| sub.status == SubscriptionStatus::Active)
            .collect()
    }

    pub fn latest_notification(&self, subscription: &Subscription) -> Notification {
        let mut entries = self.entries.clone();
        entries.sort_by(|a, b| b.pub_date.cmp(&a.pub_date));

        let entries: Vec<RssEntry> = entries.into_iter().take(5).collect();
        Notification {
            id: None,
            telegram_id: subscription.telegram_id.clone(),
            channel_name: self.title.clone(),
            channel_url: self.url.clone(),
            entries: Arc::new(entries),
            sent: false,
        }
    }

    pub fn notifications(
        &self,
        subscriptions: Vec<&Subscription>,
        entries: Arc<Vec<RssEntry>>,
    ) -> Vec<Notification> {
        subscriptions
            .iter()
            .map(|sub| Notification {
                id: None,
                telegram_id: sub.telegram_id().into(),
                channel_url: self.url().into(),
                channel_name: self.title().into(),
                entries: entries.clone(),
                sent: false,
            })
            .collect()
    }
}
