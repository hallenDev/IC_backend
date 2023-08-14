use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use local_user_index_canister::delete_account::{Response::*, *};
use types::check_jwt;
use user_index_canister::{Event as UserIndexEvent, AccountDeleted};

#[update]
fn delete_account(args: Args) -> Response {
    mutate_state(|state| delete_account_impl(args, state))
}

fn delete_account_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            let noble_id = user.noble_id;
    
            match state.data.users.remove(noble_id) {
                UpdateUserResult::Success => {
                    state.push_event_to_user_index(UserIndexEvent::AccountDeleted(Box::new(
                        AccountDeleted { noble_id }
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
    fn delete_account_test() {
        let mut runtime_state = setup_runtime_state();

        assert_eq!(runtime_state.data.users.len(), 9);

        let jwt = JWT::new(1, "".to_string(), "".to_string(), runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
        };
        let result = delete_account_impl(args, &mut runtime_state);
        assert_eq!(result, Response::Success);
       
        assert_eq!(runtime_state.data.users.len(), 8);
        assert_eq!(runtime_state.data.users.get(jwt.noble_id).is_none(), true);
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
