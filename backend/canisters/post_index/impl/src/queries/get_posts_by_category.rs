use std::cmp::Ordering;

use crate::{read_state, RuntimeState, model::post::Post};
use ic_cdk_macros::query;
use post_index_canister::get_posts_by_category::{Response::*, *};
use types::check_jwt;

#[query]
fn get_posts_by_category(args: Args) -> Response {
    read_state(|state| get_posts_by_category_impl(args, state))
}

fn get_posts_by_category_impl(
    args: Args,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    let noble_id = check_jwt(&args.jwt, now).unwrap_or_default().noble_id;

    let mut matches: Vec<&Post> = state.data.posts.iter().filter(|item| item.can_show(noble_id, &args.category, &args.following_list, &args.block_me_users)).collect();

    matches.sort_unstable_by(|lhs, rhs| {
        order_posts(&args.sort, lhs, rhs)
    });

    let total_posts_count = matches.len() as u32;

    let results = matches.iter()
        .skip(args.from as usize - 1)
        .take(args.limit as usize)
        .map(|item| item.to_summary(args.liked_posts.contains(&item.post_id), args.bookmarks.contains(&item.post_id)))
        .collect();

    Success(SuccessResult { total_posts_count, posts: results, timestamp: now })
}

fn order_posts(sort_dir: &Sort, lhs: &Post, rhs: &Post) -> Ordering {
    if *sort_dir == Sort::NewestPost {
        if lhs.date_created > rhs.date_created {
            Ordering::Less
        } else if lhs.date_created < rhs.date_created {
            Ordering::Greater
        } else if lhs.post_id < rhs.post_id {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else if *sort_dir == Sort::RecentActivity {
        if lhs.date_last_commented > rhs.date_last_commented {
            Ordering::Less
        } else if lhs.date_last_commented < rhs.date_last_commented {
            Ordering::Greater
        } else if lhs.post_id < rhs.post_id {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        Ordering::Less
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;
    use crate::model::post::Post;
    use crate::Data;
    use types::{JWT, Category, PostPrivacy, NobleId};
    use utils::env::test::TestEnv;

    #[test]
    fn search_results_page_limit() {
        let state = setup_runtime_state();

        assert_eq!(state.data.posts.len(), 6);

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(5, state.env.now()).to_string().unwrap(),
                from: 3,
                limit: 2,
                category: None,
                sort: Sort::NewestPost,
                following_list: vec![],
                block_me_users: vec![],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 1);
            assert_eq!(result.posts[0].post_id, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_results_all() {
        let state = setup_runtime_state();

        assert_eq!(state.data.posts.len(), 6);

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(5, state.env.now()).to_string().unwrap(),
                from: 1,
                limit: 5,
                category: None,
                sort: Sort::RecentActivity,
                following_list: vec![],
                block_me_users: vec![],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 3);
            assert_eq!(result.posts[0].post_id, 1);
            assert_eq!(result.posts[1].post_id, 3);
            assert_eq!(result.posts[2].post_id, 2);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn no_search_results_by_over_page() {
        let state = setup_runtime_state();

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(5, state.env.now()).to_string().unwrap(),
                from: 5,
                limit: 2,
                category: None,
                sort: Sort::NewestPost,
                following_list: vec![],
                block_me_users: vec![],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_results_by_category() {
        let state = setup_runtime_state();

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(5, state.env.now()).to_string().unwrap(),
                from: 1,
                limit: 2,
                category: Some(Category::GeneralDiscussion),
                sort: Sort::NewestPost,
                following_list: vec![],
                block_me_users: vec![],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 2);
            assert_eq!(result.posts[0].post_id, 2);
            assert_eq!(result.posts[1].post_id, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_results_by_follower() {
        let state = setup_runtime_state();

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(2, state.env.now()).to_string().unwrap(),
                from: 1,
                limit: 5,
                category: None,
                sort: Sort::NewestPost,
                following_list: vec![4 as NobleId, 1 as NobleId],
                block_me_users: vec![],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 5);
            assert_eq!(result.posts[0].post_id, 6);
            assert_eq!(result.posts[1].post_id, 2);
            assert_eq!(result.posts[2].post_id, 3);
            assert_eq!(result.posts[3].post_id, 4);
            assert_eq!(result.posts[4].post_id, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_results_by_specific_user() {
        let state = setup_runtime_state();

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(3, state.env.now()).to_string().unwrap(),
                from: 1,
                limit: 5,
                category: None,
                sort: Sort::NewestPost,
                following_list: vec![],
                block_me_users: vec![],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 4);
            assert_eq!(result.posts[0].post_id, 2);
            assert_eq!(result.posts[1].post_id, 3);
            assert_eq!(result.posts[2].post_id, 5);
            assert_eq!(result.posts[3].post_id, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn search_results_by_block_user() {
        let state = setup_runtime_state();

        let response = get_posts_by_category_impl(
            Args {
                jwt : JWT::new_for_test(5, state.env.now()).to_string().unwrap(),
                from: 1,
                limit: 5,
                category: None,
                sort: Sort::NewestPost,
                following_list: vec![4 as NobleId],
                block_me_users: vec![4 as NobleId, 2 as NobleId],
                liked_posts: vec![],
                bookmarks: vec![],
            },
            &state,
        );

        if let Success(result) = response {
            assert_eq!(result.posts.len(), 2);
            assert_eq!(result.posts[0].post_id, 3);
            assert_eq!(result.posts[1].post_id, 1);
        } else {
            assert!(false);
        }
    }

    fn setup_runtime_state() -> RuntimeState {
        let env = TestEnv::default();
        let mut data = Data::default();

        data.posts.add_test_post(Post {
            post_id: 1,
            noble_id: 1,
            category: Category::GeneralDiscussion,
            date_created: 100,
            date_last_commented: 1000,
            ..Default::default()
        });

        data.posts.add_test_post(Post {
            post_id: 2,
            noble_id: 2,
            category: Category::GeneralDiscussion,
            date_created: 200,
            date_last_commented: 200,
            ..Default::default()
        });

        data.posts.add_test_post(Post {
            post_id: 3,
            noble_id: 1,
            category: Category::IntroduceYourself,
            date_created: 200,
            date_last_commented: 500,
            ..Default::default()
        });

        data.posts.add_test_post(Post {
            post_id: 4,
            noble_id: 4,
            category: Category::IntroduceYourself,
            post_privacy: PostPrivacy::Followers,
            date_created: 200,
            date_last_commented: 500,
            ..Default::default()
        });

        let mut invited_users = HashSet::default();
        invited_users.insert(3 as NobleId);
        invited_users.insert(4 as NobleId);

        data.posts.add_test_post(Post {
            post_id: 5,
            noble_id: 1,
            category: Category::GeneralDiscussion,
            post_privacy: PostPrivacy::SpecificUsers,
            invited_users,
            date_created: 200,
            date_last_commented: 500,
            ..Default::default()
        });

        data.posts.add_test_post(Post {
            post_id: 6,
            noble_id: 1,
            category: Category::GeneralDiscussion,
            post_privacy: PostPrivacy::Followers,
            date_created: 300,
            date_last_commented: 400,
            ..Default::default()
        });

        RuntimeState::new(Box::new(env), data)
    }
}