use crate::model::user::User;
use candid::Principal;
use rand::{Rng, rngs::StdRng};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use types::{NobleId, CanisterId, TimestampMillis};
use utils::case_insensitive_hash_map::CaseInsensitiveHashMap;

#[derive(Serialize, Deserialize, Default)]
#[serde(from = "UserMapTrimmed")]
pub struct UserMap {
    users: HashMap<NobleId, User>,
    #[serde(skip)]
    username_to_noble_id: CaseInsensitiveHashMap<NobleId>,
    #[serde(skip)]
    principal_to_noble_id: HashMap<Principal, NobleId>,
    #[serde(skip)]
    email_to_noble_id: HashMap<String, NobleId>,
}

#[derive(Debug)]
pub enum UpdateUserResult {
    Success,
    UserNotFound,
}

impl UserMap {
    pub fn get(&self, noble_id: NobleId) -> Option<&User> {
        self.users.get(&noble_id)
    }

    pub fn get_mut(&mut self, noble_id: NobleId) -> Option<&mut User> {
        self.users.get_mut(&noble_id)
    }

    pub fn new_noble_id(&self, rnd: &mut StdRng) -> NobleId {
        let mut noble_id = rnd.gen_range(1000000000u64..10000000000u64);
        while self.users.get(&noble_id).is_some() {
            noble_id = rnd.gen_range(1000000000u64..10000000000u64);
        }
        noble_id
    }

    #[allow(dead_code)]
    pub fn get_canister_id_by_principal(&self, principal: &Principal) -> Option<&Principal> {
        let user = self.principal_to_noble_id.get(principal).and_then(|u| self.users.get(u));
        if user.is_none() {
            return None;
        }
        Some(&user.unwrap().canister_id)
    }

    pub fn get_by_principal(&self, principal: &Principal) -> Option<&User> {
        self.principal_to_noble_id.get(principal).and_then(|u| self.users.get(u))
    }

    pub fn get_by_email(&self, email: &str) -> Option<&User> {
        self.email_to_noble_id.get(email).and_then(|u| self.users.get(u))
    }

    pub fn get_mut_by_email(&mut self, email: &str) -> Option<&mut User> {
        self.email_to_noble_id.get(email).and_then(|u| self.users.get_mut(u))
    }

    pub fn does_username_exist(&self, username: &str) -> bool {
        self.username_to_noble_id.contains_key(username)
    }

    pub fn does_email_exist(&self, email: &str) -> bool {
        self.email_to_noble_id.contains_key(email)
    }

    pub fn get_by_username(&self, username: &str) -> Option<&User> {
        self.username_to_noble_id.get(username).and_then(|u| self.users.get(u))
    }

    pub fn len(&self) -> usize {
        self.users.len()
    }

    pub fn search(&self, term: &str) -> impl Iterator<Item = (&User, bool)> {
        self.username_to_noble_id
            .search(term)
            .filter_map(move |(uid, p)| self.users.get(uid).map(|u| (u, p)))
    }

    pub fn register(
        &mut self,
        principal: Principal,
        noble_id: NobleId,
        email: String,
        username: String,
        password: String,
        canister_id: CanisterId,
        now: TimestampMillis
    ) {
        self.username_to_noble_id.insert(&username, noble_id);
        self.principal_to_noble_id.insert(principal, noble_id);
        self.email_to_noble_id.insert(email.clone(), noble_id);

        let user = User::new(principal, noble_id, email, username, password, canister_id, now);
        self.users.insert(noble_id, user);
    }

    pub fn update_username(&mut self, previous_username: String, username: String, noble_id: NobleId) {
        self.username_to_noble_id.remove(&previous_username);
        self.username_to_noble_id.insert(&username, noble_id);
    }

    pub fn update_email(&mut self, previous_email: String, email: String, noble_id: NobleId) {
        self.email_to_noble_id.remove(&previous_email);
        self.email_to_noble_id.insert(email, noble_id);
    }

    #[allow(dead_code)]
    pub fn update(&mut self, user: User) -> UpdateUserResult {
        let noble_id = user.noble_id;

        if let Some(previous) = self.users.get(&noble_id) {
            let previous_username = &previous.username;
            let username = &user.username;
            let username_case_insensitive_changed = previous_username.to_uppercase() != username.to_uppercase();

            if username_case_insensitive_changed {
                self.username_to_noble_id.remove(previous_username);
                self.username_to_noble_id.insert(username, noble_id);
            }

            let email_changed = previous.email != user.email;
            if email_changed {
                self.email_to_noble_id.remove(&previous.email);
                self.email_to_noble_id.insert(user.email.clone(), noble_id);
            }

            self.users.insert(noble_id, user);
            UpdateUserResult::Success
        } else {
            UpdateUserResult::UserNotFound
        }
    }

    pub fn remove(&mut self, noble_id: NobleId) -> UpdateUserResult {
        if self.users.get(&noble_id).is_some() {
            self.users.remove(&noble_id);
            UpdateUserResult::Success
        } else {
            UpdateUserResult::UserNotFound
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &User> {
        self.users.values()
    }

    #[cfg(test)]
    pub fn add_test_user(&mut self, user: User) {
        self.register(
            user.principal,
            user.noble_id,
            user.email.clone(),
            user.username.clone(),
            user.password.clone(),
            user.canister_id,
            user.date_created,
        );
        self.update(user);
    }
}

#[derive(Deserialize)]
struct UserMapTrimmed {
    users: HashMap<NobleId, User>,
}

impl From<UserMapTrimmed> for UserMap {
    fn from(value: UserMapTrimmed) -> Self {
        let mut user_map = UserMap {
            users: value.users,
            ..Default::default()
        };

        for (noble_id, user) in user_map.users.iter_mut() {
            user_map.username_to_noble_id.insert(&user.username, *noble_id);
            user_map.principal_to_noble_id.insert(user.principal, *noble_id);
            user_map.email_to_noble_id.insert(user.email.clone(), *noble_id);
        }

        user_map
    }
}
