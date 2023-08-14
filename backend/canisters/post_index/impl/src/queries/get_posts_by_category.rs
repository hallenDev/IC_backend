use std::cmp::Ordering;

use crate::{read_state, RuntimeState, model::post::Post};
use ic_cdk_macros::query;
use post_index_canister::get_posts_by_category::{Response::*, *};
use types::{Category, PostPrivacy, check_jwt, NobleId};

#[query]
async fn get_posts_by_category(args: Args) -> Response {
    let (user_index_canister_id, now) = read_state(|state| (state.data.user_index_canister_id, state.env.now()));

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        let local_user_index_canister_id = match user_index_canister_c2c_client::c2c_get_local_user_index_canister_id(
            user_index_canister_id,
            &user_index_canister::c2c_get_local_user_index_canister_id::Args{noble_id: jwt.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    user_index_canister::c2c_get_local_user_index_canister_id::Response::Success(id) => id,
                    user_index_canister::c2c_get_local_user_index_canister_id::Response::UserNotFound => return PermissionDenied,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        };

        let following_list = match local_user_index_canister_c2c_client::c2c_get_following_list(
            local_user_index_canister_id,
            &local_user_index_canister::c2c_get_following_list::Args{noble_id: jwt.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    local_user_index_canister::c2c_get_following_list::Response::Success(list) => list,
                    local_user_index_canister::c2c_get_following_list::Response::UserNotFound => return PermissionDenied,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        };

        let block_me_users = match local_user_index_canister_c2c_client::c2c_get_block_me_users(
            local_user_index_canister_id,
            &local_user_index_canister::c2c_get_block_me_users::Args{noble_id: jwt.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    local_user_index_canister::c2c_get_block_me_users::Response::Success(list) => list,
                    local_user_index_canister::c2c_get_block_me_users::Response::UserNotFound => return PermissionDenied,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        };

        let block_users = match local_user_index_canister_c2c_client::c2c_get_block_users(
            local_user_index_canister_id,
            &local_user_index_canister::c2c_get_block_users::Args{noble_id: jwt.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    local_user_index_canister::c2c_get_block_users::Response::Success(list) => list,
                    local_user_index_canister::c2c_get_block_users::Response::UserNotFound => return PermissionDenied,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        };

        read_state(|state| get_posts_by_category_impl(args, jwt.noble_id, following_list, block_me_users, block_users, state))
    } else {
        PermissionDenied
    }
}

fn get_posts_by_category_impl(
    args: Args,
    noble_id: NobleId,
    following_list: Vec<NobleId>,
    block_me_users: Vec<NobleId>,
    block_users: Vec<NobleId>,
    state: &RuntimeState
) -> Response {
    let now = state.env.now();

    let mut matches: Vec<&Post> = state.data.posts.iter().filter(|item| is_filtered(item, noble_id, &args.category, &following_list, &block_me_users, &block_users)).collect();

    matches.sort_unstable_by(|lhs, rhs| {
        order_messages(&args.sort, lhs, rhs)
    });

    let from = std::cmp::min(((args.page - 1) * args.limit )as usize, matches.len());

    let results = matches[from..]
        .iter()
        .take(args.limit as usize)
        .map(|item| item.to_summary())
        .collect();

    Success(SuccessResult { total_posts_count: matches.len() as u32, posts: results, timestamp: now })
}

fn order_messages(sort_dir: &Sort, lhs: &Post, rhs: &Post) -> Ordering {
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

fn is_filtered(
    post: &Post,
    noble_id: NobleId,
    category: &Option<Category>,
    following_list: &Vec<NobleId>,
    block_me_users: &Vec<NobleId>,
    block_users: &Vec<NobleId>,
) -> bool {
    if block_me_users.contains(&post.owner) || block_users.contains(&post.owner) {
        return false;
    }

    if post.post_privacy == PostPrivacy::AnyBody ||
      (post.post_privacy == PostPrivacy::Followers && following_list.contains(&post.owner)) ||
      (post.post_privacy == PostPrivacy::SpecificUsers && post.invited_users.contains(&noble_id)) {
        if let Some(cate) = category {
            *cate == post.category
        } else {
            true
        }
    } else {
        false
    }
}
