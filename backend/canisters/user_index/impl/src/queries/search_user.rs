use crate::model::user::User;
use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use user_index_canister::search_user::{Response::*, *};
use types::{NobleId, check_jwt};

#[query]
fn search_user(args: Args) -> Response {
    read_state(|state| search_user_impl(args, state))
}

fn search_user_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        let users = &state.data.users;
        let mut search_term = args.search_term;
        search_term = search_term.trim().to_string().to_lowercase();
    
        let matches: Vec<&User> = users.iter().filter(|item| is_filtered(item, &search_term, jwt.noble_id, &args.block_me_users, &args.exclude_users)).collect();
    
        // Page
        let results = matches
            .iter()
            .take(args.max_results as usize)
            .map(|u| u.to_summary(args.following_list.contains(&u.noble_id)))
            .collect();
    
        Success(SuccessResult {
            users: results,
            timestamp: now,
        })
    } else {
        PermissionDenied
    }
}

fn is_filtered(
    user: &User,
    search_term: &str,
    noble_id: NobleId,
    block_me_users: &Vec<NobleId>,
    exclude_users: &Vec<NobleId>
) -> bool {
    if user.noble_id == noble_id {
        return false;
    }
    if exclude_users.contains(&user.noble_id) {
        return false;
    }
    if block_me_users.contains(&user.noble_id) {
        return false;
    }
    if user.username.is_empty() {
        return false;
    }
    if format!("{} {}", user.first_name, user.last_name).to_lowercase().contains(search_term) {
        return true;
    }
    if user.search_by_email && user.email.contains(search_term) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::user::User;
    use crate::Data;
    use candid::Principal;
    use types::JWT;
    use utils::env::test::TestEnv;

    #[test]
    fn search_results_constrained_by_max_results() {
        let state = setup_runtime_state();

        assert_eq!(state.data.users.len(), 4);

        let response = search_user_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 1,
                search_term: "viktor".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(1, results.users.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_results_by_name() {
        let state = setup_runtime_state();

        let response = search_user_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "viktor".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(2, results.users.len());
        } else {
            assert!(false);
        }
    }


    #[test]
    fn search_results_by_email() {
        let state = setup_runtime_state();

        let response = search_user_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "rustdev".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(1, results.users.len());
            assert_eq!(2, results.users[0].noble_id);
        } else {
            assert!(false);
        }
    }

    fn setup_runtime_state() -> RuntimeState {
        let mut env = TestEnv::default();
        let mut data = Data::default();

        data.users.add_test_user(User {
            principal: Principal::from_slice(&[1]),
            noble_id: 1,
            username: "me".to_string(),
            date_created: env.now,
            ..Default::default()
        });

        env.now += 100;

        data.users.add_test_user(User {
            principal: Principal::from_slice(&[2]),
            noble_id: 2,
            username: "user1".to_string(),
            first_name: "Viktor ".to_string(),
            last_name: "Vasylchuk".to_string(),
            email: "rustdev@gmail.com".to_string(),
            search_by_email: true,
            date_created: env.now,
            ..Default::default()
        });

        env.now += 100;

        data.users.add_test_user(User {
            principal: Principal::from_slice(&[3]),
            noble_id: 3,
            username: "user2".to_string(),
            first_name: "Viktor ".to_string(),
            last_name: "Koval".to_string(),
            email: "reactdev@gmail.com".to_string(),
            search_by_email: false,
            date_created: env.now,
            ..Default::default()
        });

        env.now += 100;

        data.users.add_test_user(User {
            principal: Principal::from_slice(&[4]),
            noble_id: 4,
            username: "user3".to_string(),
            first_name: "Vladyslav ".to_string(),
            last_name: "Melnyk".to_string(),
            email: "rustdev99@gmail.com".to_string(),
            search_by_email: false,
            date_created: env.now,
            ..Default::default()
        });

        env.now += 100;

        RuntimeState::new(Box::new(env), data)
    }
}