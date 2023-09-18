use crate::model::user::User;
use crate::{read_state, RuntimeState};
use core::cmp::Ordering;
use ic_cdk_macros::query;
use user_index_canister::search_user_by_username::{Response::*, *};
use types::{check_jwt, NobleId};

const MAX_SEARCH_TERM_LENGTH: usize = 25;

#[query]
fn search_user_by_username(args: Args) -> Response {
    read_state(|state| search_user_by_username_impl(args, state))
}

fn search_user_by_username_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        let users = &state.data.users;
    
        // Remove spaces since usernames can't have spaces
        let mut search_term = args.search_term.replace(' ', "");
        search_term.truncate(MAX_SEARCH_TERM_LENGTH);

        // Filter
        let mut matches: Vec<(&User, bool)> = users.search(&search_term).filter(|(u, _)| is_filtered(*u, jwt.noble_id, &args.block_me_users, &args.exclude_users)).collect();
    
        // Sort
        matches.sort_unstable_by(|(u1, u1_starts_ci), (u2, u2_starts_ci)| {
            order_usernames(&search_term, &u1.username, *u1_starts_ci, &u2.username, *u2_starts_ci)
        });
    
        // Page
        let results = matches
            .iter()
            .take(args.max_results as usize)
            .map(|(u, _)| u.to_summary(args.following_list.contains(&u.noble_id)))
            .collect();
    
        Success(SuccessResult {
            users: results,
            timestamp: now,
        })
    } else {
        PermissionDenied
    }
}

fn is_filtered(user: &User, noble_id: NobleId, block_me_users: &Vec<NobleId>, exclude_users: &Vec<NobleId>) -> bool {
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
    true
}

fn order_usernames(search_term: &str, u1: &str, u1_starts_ci: bool, u2: &str, u2_starts_ci: bool) -> Ordering {
    // First check case insensitive match
    if u1_starts_ci && !u2_starts_ci {
        Ordering::Less
    } else if !u1_starts_ci && u2_starts_ci {
        Ordering::Greater
    } else {
        // Now order by shortest username first
        match u1.len().cmp(&u2.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                if u1_starts_ci {
                    // Now prioritise case sensitive prefix match
                    let u1_starts = u1.starts_with(search_term);
                    let u2_starts = u2.starts_with(search_term);
                    if u1_starts != u2_starts {
                        if u1_starts {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                } else {
                    // Now prioritise case sensitive contains match
                    let u1_contains = u1.contains(search_term);
                    let u2_contains = u2.contains(search_term);
                    if u1_contains != u2_contains {
                        if u1_contains {
                            return Ordering::Less;
                        } else {
                            return Ordering::Greater;
                        }
                    }
                }

                // Finally order the matches alphabetically
                u1.cmp(u2)
            }
        }
    }
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

        let response = search_user_by_username_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 2,
                search_term: "ma".to_string(),
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
    fn case_insensitive_matches() {
        let state = setup_runtime_state();

        let response = search_user_by_username_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "MA".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(6, results.users.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn results_ordered_by_all_criteria() {
        let state = setup_runtime_state();

        let response = search_user_by_username_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "Ma".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(2, results.users[0].noble_id);
            assert_eq!(0, results.users[1].noble_id);
            assert_eq!(6, results.users[2].noble_id);
            assert_eq!(7, results.users[3].noble_id);
            assert_eq!(8, results.users[4].noble_id);
            assert_eq!(5, results.users[5].noble_id);
        } else {
            assert!(false);
        }
    }


    #[test]
    fn block_user() {
        let state = setup_runtime_state();

        let response = search_user_by_username_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "Ma".to_string(),
                following_list: vec![2],
                block_me_users: vec![5, 6, 8],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(2, results.users[0].noble_id);
            assert_eq!(true, results.users[0].follow_state);
            assert_eq!(0, results.users[1].noble_id);
            assert_eq!(false, results.users[1].follow_state);
            assert_eq!(7, results.users[2].noble_id);
            assert_eq!(false, results.users[2].follow_state);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_with_zero_length_term_matches_all_users() {
        let state = setup_runtime_state();

        let response = search_user_by_username_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            assert_eq!(8, results.users.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn all_fields_set_correctly() {
        let state = setup_runtime_state();

        let response = search_user_by_username_impl(
            Args {
                jwt : JWT::new_for_test(1, state.env.now()).to_string().unwrap(),
                max_results: 10,
                search_term: "hamish".to_string(),
                following_list: vec![],
                block_me_users: vec![],
                exclude_users: vec![],
            },
            &state,
        );

        if let Response::Success(results) = response {
            let user = results.users.first().unwrap();
            assert_eq!(user.noble_id, 4);
        } else {
            assert!(false);
        }
    }

    fn setup_runtime_state() -> RuntimeState {
        let mut env = TestEnv::default();
        let mut data = Data::default();

        let usernames = vec![
            "Martin", "marcus", "matty", "julian", "hamish", "mohammad", "amar", "muhamMad", "amabcdef",
        ];

        for (index, username) in usernames.iter().enumerate() {
            let bytes = vec![index as u8];
            let p = Principal::from_slice(&bytes[..]);

            data.users.add_test_user(User {
                principal: p,
                noble_id: index as u64,
                username: username.to_string(),
                date_created: env.now,
                ..Default::default()
            });
            env.now += 1000;
        }

        RuntimeState::new(Box::new(env), data)
    }
}
