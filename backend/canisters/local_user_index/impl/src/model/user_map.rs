use crate::model::user::User;
use candid::Principal;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use types::{TimestampMillis, NobleId};
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

    // pub fn get_by_principal(&self, principal: &Principal) -> Option<&User> {
    //     self.principal_to_noble_id.get(principal).and_then(|u| self.users.get(u))
    // }

    // pub fn does_username_exist(&self, username: &str) -> bool {
    //     self.username_to_noble_id.contains_key(username)
    // }

    #[allow(dead_code)]
    pub fn get_by_username(&self, username: &str) -> Option<&User> {
        self.username_to_noble_id.get(username).and_then(|u| self.users.get(u))
    }

    pub fn len(&self) -> usize {
        self.users.len()
    }

    pub fn register(
        &mut self,
        principal: Principal,
        noble_id: NobleId,
        email: String,
        username: String,
        password_hash: String,
        now: TimestampMillis,
    ) {
        self.username_to_noble_id.insert(&username, noble_id);
        self.principal_to_noble_id.insert(principal, noble_id);
        self.email_to_noble_id.insert(email.clone(), noble_id);

        let user = User::new(principal, noble_id, email, username, password_hash, now);
        self.users.insert(noble_id, user);
    }

    pub fn update(&mut self, mut user: User, now: TimestampMillis) -> UpdateUserResult {
        let noble_id = user.noble_id;

        if let Some(previous) = self.users.get(&noble_id) {
            let previous_username = &previous.username;
            let username = &user.username;
            let username_case_insensitive_changed = previous_username.to_uppercase() != username.to_uppercase();

            user.date_updated = now;

            if username_case_insensitive_changed {
                self.username_to_noble_id.remove(previous_username);
                self.username_to_noble_id.insert(username, noble_id);
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

    #[cfg(test)]
    pub fn add_test_user(&mut self, user: User) {
        let date_created = user.date_created;
        self.register(
            user.principal,
            user.noble_id,
            user.email.clone(),
            user.username.clone(),
            user.password.clone(),
            user.date_created,
        );
        self.update(user, date_created);
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

        for (noble_id, user) in user_map.users.iter() {
            user_map.username_to_noble_id.insert(&user.username, *noble_id);
            user_map.principal_to_noble_id.insert(user.principal, *noble_id);
        }

        user_map
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn register_with_no_clashes() {
        let mut user_map = UserMap::default();
        let principal1 = Principal::from_slice(&[1]);
        let principal2 = Principal::from_slice(&[2]);
        let principal3 = Principal::from_slice(&[3]);

        let username1 = "1".to_string();
        let username2 = "2".to_string();
        let username3 = "3".to_string();

        let noble_id1: NobleId = 1;
        let noble_id2: NobleId = 2;
        let noble_id3: NobleId = 3;

        let user1 = User {
            principal: principal1,
            noble_id: noble_id1,
            username: username1.clone(),
            date_created: 1,
            date_updated: 1,
            ..Default::default()
        };

        let user2 = User {
            principal: principal2,
            noble_id: noble_id2,
            username: username2.clone(),
            date_created: 2,
            date_updated: 2,
            ..Default::default()
        };

        let user3 = User {
            principal: principal3,
            noble_id: noble_id3,
            username: username3.clone(),
            date_created: 3,
            date_updated: 3,
            ..Default::default()
        };

        user_map.add_test_user(user1);
        user_map.add_test_user(user2);
        user_map.add_test_user(user3);

        let principal_to_noble_id: Vec<_> = user_map
            .principal_to_noble_id
            .iter()
            .map(|(p, u)| (*p, *u))
            .sorted_by_key(|(_, u)| *u)
            .collect();
        let username_to_noble_id: Vec<_> = user_map
            .username_to_noble_id
            .iter()
            .map(|(name, u)| (name.clone(), *u))
            .sorted_by_key(|(_, u)| *u)
            .collect();

        assert_eq!(user_map.users.len(), 3);

        assert_eq!(
            username_to_noble_id,
            vec!((username1, noble_id1), (username2, noble_id2), (username3, noble_id3))
        );
        assert_eq!(
            principal_to_noble_id,
            vec!((principal1, noble_id1), (principal2, noble_id2), (principal3, noble_id3))
        );
    }

    #[test]
    fn update_with_no_clashes() {
        let mut user_map = UserMap::default();
        let principal = Principal::from_slice(&[1]);

        let username1 = "1".to_string();
        let username2 = "2".to_string();

        let noble_id = 1;

        let user = User {
            principal: principal,
            noble_id: noble_id,
            username: username1,
            date_created: 1,
            date_updated: 1,
            ..Default::default()
        };
        
        user_map.add_test_user(user);

        if let Some(original) = user_map.get(noble_id) {
            let mut updated = original.clone();
            updated.username = username2.clone();

            assert!(matches!(user_map.update(updated, 3), UpdateUserResult::Success));

            assert_eq!(user_map.users.keys().collect_vec(), vec!(&noble_id));
            assert_eq!(user_map.username_to_noble_id.len(), 1);
            assert!(user_map.username_to_noble_id.contains_key(&username2));
            assert_eq!(user_map.principal_to_noble_id.keys().collect_vec(), vec!(&principal));
        }
    }

    #[test]
    fn update_username_change_casing() {
        let mut user_map = UserMap::default();
        let principal = Principal::from_slice(&[1]);
        let username = "abc".to_string();
        let noble_id = 1;

        let original = User {
            principal,
            noble_id,
            username,
            date_created: 1,
            date_updated: 1,
            ..Default::default()
        };

        let mut updated = original.clone();

        user_map.add_test_user(original);
        updated.username = "ABC".to_string();

        assert!(matches!(user_map.update(updated, 2), UpdateUserResult::Success));
    }
}
