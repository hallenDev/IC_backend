use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use local_user_index_canister::set_search_by_email::{Response::*, *};
use user_index_canister::{Event as UserIndexEvent, SearchByEmailChanged};
use types::check_jwt;

#[update]
fn set_search_by_email(args: Args) -> Response {
    mutate_state(|state| set_search_by_email_impl(args, state))
}

fn set_search_by_email_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            let mut user_to_update = user.clone();
    
            user_to_update.search_by_email = args.search_by_email;
    
            let now = state.env.now();
    
            match state.data.users.update(user_to_update, now) {
                UpdateUserResult::Success => {
                    state.push_event_to_user_index(UserIndexEvent::SearchByEmailChanged(Box::new(
                        SearchByEmailChanged { noble_id: jwt.noble_id, search_by_email: args.search_by_email }
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
    fn set_search_by_email_test() {
        let mut runtime_state = setup_runtime_state();

        let jwt = JWT::new(1, "".to_string(), "".to_string(), runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            search_by_email: true,
        };
        let result = set_search_by_email_impl(args, &mut runtime_state);
        assert_eq!(result, Response::Success);

        let user = runtime_state.data.users.get(jwt.noble_id).unwrap();
        assert_eq!(user.search_by_email, true);

        let jwt = JWT::new(1, "".to_string(), "".to_string(), runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            search_by_email: false,
        };
        let result = set_search_by_email_impl(args, &mut runtime_state);
        assert_eq!(result, Response::Success);

        let user = runtime_state.data.users.get(jwt.noble_id).unwrap();
        assert_eq!(user.search_by_email, false);
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
