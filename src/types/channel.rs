use std::sync::Arc;

use super::{Channel, Notification, RssEntry, Subscription, SubscriptionStatus};

impl Channel {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn update_entries(&mut self, entries: &Arc<Vec<RssEntry>>) {
        self.entries = Arc::clone(entries);
    }

    pub fn released_entries(&self) -> Vec<RssEntry> {
        let cloned_entires = (*self.entries).clone();

        cloned_entires
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
        let mut cloned_entries = (*self.entries).clone();
        cloned_entries.sort_unstable_by(|a, b| b.pub_date.cmp(&a.pub_date));

        let entries: Vec<RssEntry> = cloned_entries.into_iter().take(5).collect();
        Notification {
            id: None,
            telegram_id: subscription.telegram_id().into(),
            channel_name: self.title.clone(),
            channel_url: self.url.clone(),
            entries: Arc::new(entries),
            sent: false,
        }
    }

    pub fn notifications(
        &self,
        subscriptions: Vec<&Subscription>,
        entries: &Arc<Vec<RssEntry>>,
    ) -> Vec<Notification> {
        subscriptions
            .iter()
            .map(|sub| Notification {
                id: None,
                telegram_id: sub.telegram_id().into(),
                channel_url: self.url().into(),
                channel_name: self.title().into(),
                entries: Arc::clone(entries),
                sent: false,
            })
            .collect()
    }
}
