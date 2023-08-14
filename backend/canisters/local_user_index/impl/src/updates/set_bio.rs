use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState, MAX_BIO_LENGTH};
use ic_cdk_macros::update;
use local_user_index_canister::set_bio::{Response::*, *};
use user_index_canister::{Event as UserIndexEvent, BioChanged};
use types::check_jwt;
use utils::truncate_string::truncate_string;

#[update]
fn set_bio(args: Args) -> Response {
    mutate_state(|state| set_bio_impl(args, state))
}

fn set_bio_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if args.bio.len() > MAX_BIO_LENGTH {
            return BioTooLong(MAX_BIO_LENGTH);
        }
    
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            let mut user_to_update = user.clone();
            user_to_update.bio = args.bio.clone();
            let now = state.env.now();
    
            match state.data.users.update(user_to_update, now) {
                UpdateUserResult::Success => {
                    state.push_event_to_user_index(UserIndexEvent::BioChanged(Box::new(
                        BioChanged { noble_id: jwt.noble_id, bio: truncate_string(args.bio, 100) }
                    )));
                    Success
                },
                UpdateUserResult::UserNotFound => UserNotFound,
            }
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Data;
    use crate::model::user::User;
    use candid::Principal;
    use types::{NobleId, JWT};
    use utils::env::test::TestEnv;

    #[test]
    fn set_bio_test() {
        let mut runtime_state = setup_runtime_state();

        let jwt = JWT::new(1, "".to_string(), "".to_string(), runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            bio: "This is my bio".to_string(),
        };
        let result = set_bio_impl(args, &mut runtime_state);
        assert_eq!(result, Response::Success);

        let user = runtime_state.data.users.get(jwt.noble_id).unwrap();
        assert_eq!(user.bio, "This is my bio".to_string());
    }

    fn setup_runtime_state() -> RuntimeState {
        let mut env = TestEnv::default();
        let mut data = Data::default();

        let usernames = vec![
            "Martin", "marcus", "matty", "julian", "hamish", "mohammad", "amar", "muhamMad", "amabcdef",
        ];

        for (index, username) in usernames.iter().enumerate() {
            let bytes = vec![index as u8];
            let p = Principal::from_slice(&bytes);

            data.users.add_test_user(User {
                principal: p,
                noble_id: index as NobleId,
                username: username.to_string(),
                date_created: env.now,
                date_updated: env.now,
                ..Default::default()
            });
            env.now += 1000;
        }

        RuntimeState::new(Box::new(env), data)
    }
}
