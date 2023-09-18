use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, VecDeque};

#[derive(Serialize, Deserialize)]
pub struct EmailEventSyncQueue<T> {
    queue: VecDeque<String>,
    sync_in_progress: bool,
    events: HashMap<String, Vec<T>>,
    max_canisters_per_batch: usize,
    max_events_per_canister_per_batch: usize,
}

impl<T> Default for EmailEventSyncQueue<T> {
    fn default() -> EmailEventSyncQueue<T> {
        EmailEventSyncQueue {
            queue: VecDeque::default(),
            sync_in_progress: false,
            events: HashMap::default(),
            max_canisters_per_batch: 10,
            max_events_per_canister_per_batch: 1000,
        }
    }
}

impl<T> EmailEventSyncQueue<T> {
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn sync_in_progress(&self) -> bool {
        self.sync_in_progress
    }

    pub fn push(&mut self, email: String, event: T) {
        match self.events.entry(email.clone()) {
            Vacant(e) => {
                self.queue.push_back(email);
                e.insert(vec![event]);
            }
            Occupied(e) => {
                e.into_mut().push(event);
            }
        }
    }

    pub fn try_start_single(&mut self) -> Option<(String, Vec<T>)> {
        if self.sync_in_progress {
            return None;
        }

        let email = self.queue.pop_front()?;
        if let Some((events, has_more_events)) = self.take_events(email.clone()) {
            self.sync_in_progress = true;
            if has_more_events {
                self.queue.push_back(email.clone());
            }
            Some((email, events))
        } else {
            None
        }
    }

    pub fn try_start_batch(&mut self) -> Option<Vec<(String, Vec<T>)>> {
        if self.sync_in_progress || self.queue.is_empty() {
            None
        } else {
            let mut batch = Vec::new();
            let mut canisters_to_readd = Vec::new();
            while let Some(email) = self.queue.pop_front() {
                if let Some((events, has_more_events)) = self.take_events(email.clone()) {
                    if has_more_events {
                        // If there are more events, queue up the canister to be processed again
                        canisters_to_readd.push(email.clone());
                    }
                    batch.push((email, events));
                    if batch.len() >= self.max_canisters_per_batch {
                        break;
                    }
                }
            }
            for email in canisters_to_readd {
                self.queue.push_back(email);
            }
            if batch.is_empty() {
                None
            } else {
                self.sync_in_progress = true;
                Some(batch)
            }
        }
    }

    pub fn mark_batch_completed(&mut self) {
        self.sync_in_progress = false;
    }

    pub fn mark_sync_failed_for_canister(&mut self, email: String, events: Vec<T>) {
        let merged_events = match self.events.remove_entry(&email) {
            Some((_, old_events)) => events.into_iter().chain(old_events).collect(),
            None => {
                self.queue.push_back(email.clone());
                events
            }
        };

        self.events.insert(email, merged_events);
    }

    fn take_events(&mut self, email: String) -> Option<(Vec<T>, bool)> {
        if let Occupied(mut e) = self.events.entry(email) {
            let vec = e.get_mut();
            let count = min(vec.len(), self.max_events_per_canister_per_batch);
            if count == 0 {
                return None;
            }

            let mut items = Vec::with_capacity(count);
            for item in vec.drain(..count) {
                items.push(item);
            }

            let has_more_events = !vec.is_empty();
            if !has_more_events {
                // If there are no more events, remove the entry from the map
                e.remove();
            }

            Some((items, has_more_events))
        } else {
            None
        }
    }
}
