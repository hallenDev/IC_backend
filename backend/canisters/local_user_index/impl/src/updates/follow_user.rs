use crate::{mutate_state, RuntimeState, read_state};
use ic_cdk_macros::update;
use local_user_index_canister::follow_user::{Response::*, *};
use types::{check_jwt, NobleId};
use user_index_canister::{Event as UserIndexEvent, FollowUser};

#[update]
async fn follow_user(args: Args) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, read_state(|state| state.env.now())) {
        let user_index_canister_id = read_state(|state| state.data.user_index_canister_id);

        match user_index_canister_c2c_client::c2c_is_nobleblocks_user(
            user_index_canister_id,
            &user_index_canister::c2c_is_nobleblocks_user::Args{noble_id: args.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    user_index_canister::c2c_is_nobleblocks_user::Response::Yes => {},
                    user_index_canister::c2c_is_nobleblocks_user::Response::No => return UserNotFound,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        }
    
        mutate_state(|state| follow_user_impl(jwt.noble_id, args, state))
    } else {
        PermissionDenied
    }
}

fn follow_user_impl(noble_id: NobleId, args: Args, state: &mut RuntimeState) -> Response {
    if let Some(sender) = state.data.users.get_mut(noble_id) {
        let sender_id = noble_id;
        let receiver_id = args.noble_id;
        if sender.is_following(receiver_id) {
            return AlreadyFollowing;
        }

        if sender_id == receiver_id {
            return UserNotFound;
        }

        sender.add_following_user(args.noble_id);

        if let Some(receiver) = state.data.users.get_mut(receiver_id) {
            receiver.add_follower(sender_id);
        } else {
            state.push_event_to_user_index(UserIndexEvent::UserFollowed(Box::new(
                FollowUser { sender_id, receiver_id }
            )));
        }
        Success
    } else {
        UserNotFound
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
    fn success() {
        let mut runtime_state = setup_runtime_state();

        let jwt = JWT::new_for_test(1, runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            noble_id: 3,
        };
        assert_eq!(runtime_state.data.users.get(1).unwrap().is_following(3), false);
        assert_eq!(runtime_state.data.users.get(3).unwrap().is_follower(1), false);
        let result = follow_user_impl(jwt.noble_id, args, &mut runtime_state);
        assert_eq!(result, Response::Success);
        assert_eq!(runtime_state.data.users.get(1).unwrap().is_following(3), true);
        assert_eq!(runtime_state.data.users.get(3).unwrap().is_follower(1), true);
    }

    #[test]
    fn already_following() {
        let mut runtime_state = setup_runtime_state();

        let jwt = JWT::new_for_test(1, runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            noble_id: 4,
        };
        let result = follow_user_impl(jwt.noble_id, args, &mut runtime_state);
        assert_eq!(result, Response::Success);

        let jwt = JWT {
            noble_id: 1,
            iat: runtime_state.env.now(),
            exp: runtime_state.env.now() + 60,
            ..Default::default()
        };
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            noble_id: 4,
        };
        let result = follow_user_impl(jwt.noble_id, args, &mut runtime_state);
        assert_eq!(result, Response::AlreadyFollowing);
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
