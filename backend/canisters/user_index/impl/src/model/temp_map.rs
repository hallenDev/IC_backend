use rand::{Rng, rngs::StdRng};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use types::{TimestampMillis, TempId};
use crate::model::temp::{Temp, TempData};


#[derive(Serialize, Deserialize, Default)]
pub struct TempMap {
    temps: HashMap<TempId, Temp>,
}

pub const TEMP_EXPIRED_DURATION: TimestampMillis = 3 * 60 * 1000; // 3 mins.
pub const AVAILABLE_RESEND_DURATION: TimestampMillis = 30 * 1000; // 30 secs.

impl TempMap {
    pub fn get(&self, temp_id: TempId) -> Option<&Temp> {
        self.temps.get(&temp_id)
    }

    pub fn get_mut(&mut self, temp_id: TempId) -> Option<&mut Temp> {
        self.temps.get_mut(&temp_id)
    }

    pub fn remove_expired_temp(&mut self, now: TimestampMillis) {
        let temp_ids: Vec<TempId> = self.temps.iter().filter(|item| item.1.expired_time < now).map(|item| *item.0).collect();
        temp_ids.iter().for_each(|item| {
            self.temps.remove(item);
        });
    }

    pub fn remove(&mut self, temp_id: TempId) {
        self.temps.remove(&temp_id);
    }

    pub fn new_passkey(&self, rnd: &mut StdRng) -> String {
        // let mut passkey: String = rnd.sample_iter(&Alphanumeric).take(6).map(char::from).collect();
        let mut passkey = String::new();
        for _ in 0..6 {
            passkey += &rnd.gen_range(0..10).to_string();
        }
        passkey
    }

    pub fn add_new_temp(&mut self, email: String, temp_data: TempData, rnd: &mut StdRng, now: TimestampMillis) -> (TempId, String) {
        let mut new_temp_id = rnd.gen_range(100000000u32..1000000000u32);
        while self.temps.get(&new_temp_id).is_some() {
            new_temp_id = rnd.gen_range(100000000u32..1000000000u32);
        }
        let passkey = self.new_passkey(rnd);

        self.temps.insert(new_temp_id, Temp {
            temp_id: new_temp_id,
            is_used: false,
            expired_time: now + TEMP_EXPIRED_DURATION,
            email,
            passkey: passkey.clone(),
            temp_data,
        });

        (new_temp_id, passkey)
    }

    pub fn does_username_exist(&self, username: &str) -> bool {
        self.temps.iter().any(|item| {
            match &item.1.temp_data {
                TempData::RegisterUser(user) => user.username == username,
                _ => false,
            }
        })
    }

    pub fn does_email_exist(&self, email: &str) -> bool {
        self.temps.iter().any(|item| {
            match &item.1.temp_data {
                TempData::RegisterUser(user) => user.email == email,
                _ => false,
            }
        })
    }

    pub fn does_exist(&mut self, email: &str, username: &str, rnd: &mut StdRng, now: TimestampMillis) -> Option<(TempId, String)> {
        let mut result = None;
        let passkey = self.new_passkey(rnd);
        self.temps.iter_mut().for_each(|item| {
            match &item.1.temp_data {
                TempData::RegisterUser(user) => {
                    if user.email == email && user.username == username {
                        item.1.expired_time = now + TEMP_EXPIRED_DURATION;
                        item.1.is_used = false;
                        item.1.passkey = passkey.clone();
                        result = Some((*item.0, passkey.clone()));
                    }
                },
                _ => {},
            }
        });
        result
    }
}
