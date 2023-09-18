use crate::model::user::User;
use candid::Principal;
use rand::rngs::StdRng;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use types::{TimestampMillis, NobleId, CanisterId, AvatarId};

#[derive(Serialize, Deserialize, Default)]
#[serde(from = "UserMapTrimmed")]
pub struct UserMap {
    users: HashMap<NobleId, User>,
    #[serde(skip)]
    pub avatar_id_to_noble_id: HashMap<AvatarId, NobleId>,
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

    pub fn len(&self) -> usize {
        self.users.len()
    }

    pub fn register(
        &mut self,
        principal: Principal,
        noble_id: NobleId,
        canister_id: CanisterId,
        email: String,
        username: String,
        now: TimestampMillis,
    ) {
        let user = User::new(principal, noble_id, canister_id, email, username, now);
        self.users.insert(noble_id, user);
    }

    pub fn remove(&mut self, noble_id: NobleId) -> UpdateUserResult {
        if self.users.get(&noble_id).is_some() {
            self.users.remove(&noble_id);
            UpdateUserResult::Success
        } else {
            UpdateUserResult::UserNotFound
        }
    }

    pub fn update_avatar_id(&mut self, noble_id: NobleId, rng: &mut StdRng) -> AvatarId {
        let mut avatar_id = utils::env::get_random_id(rng);
        while self.avatar_id_to_noble_id.contains_key(&avatar_id) {
            avatar_id = utils::env::get_random_id(rng);
        }

        if let Some(user) = self.get_mut(noble_id) {
            let older_avatar_id = user.avatar_id;
            user.avatar_id = avatar_id;

            self.avatar_id_to_noble_id.remove(&older_avatar_id);
            self.avatar_id_to_noble_id.insert(avatar_id, noble_id);

            avatar_id
        } else {
            0
        }
    }

    #[cfg(test)]
    pub fn add_test_user(&mut self, user: User) {
        self.register(
            user.principal,
            user.noble_id,
            user.canister_id,
            user.email.clone(),
            user.username.clone(),
            user.date_created,
        );
        self.update(user);
    }

    #[cfg(test)]
    pub fn update(&mut self, user: User) -> UpdateUserResult {
        let noble_id = user.noble_id;

        if let Some(_) = self.users.get(&noble_id) {
            self.users.insert(noble_id, user);
            UpdateUserResult::Success
        } else {
            UpdateUserResult::UserNotFound
        }
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
            let avatar_id = user.avatar_id;
            if avatar_id != 0 {
                user_map.avatar_id_to_noble_id.insert(avatar_id, *noble_id);
                user.avatar_id = avatar_id;
            }
        }

        user_map
    }
}
