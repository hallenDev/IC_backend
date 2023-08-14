use crate::{mutate_state, RuntimeState, read_state, MAX_TITLE_LENGTH, MAX_DESCRIPTION_LENGTH};
use ic_cdk_macros::update;
use rand::Rng;
use types::{CanisterId, NobleId, check_jwt, PostId, TimestampMillis};
use post_index_canister::new_post::{Response::*, *};
use utils::truncate_string::truncate_string;

#[update]
async fn new_post(args: Args) -> Response {
    let (user_index_canister_id, now) = read_state(|state| (state.data.user_index_canister_id, state.env.now()));

    if let Some(jwt) = check_jwt(&args.jwt, now) {
        match user_index_canister_c2c_client::c2c_is_nobleblocks_user(
            user_index_canister_id,
            &user_index_canister::c2c_is_nobleblocks_user::Args{noble_id: jwt.noble_id}
        ).await {
            Ok(response) => {
                match response {
                    user_index_canister::c2c_is_nobleblocks_user::Response::Yes => {},
                    user_index_canister::c2c_is_nobleblocks_user::Response::No => return PermissionDenied,
                }
            },
            Err(error) => return InternalError(format!("{:?}", error)),
        }

        let (canister_id, post_id) = match mutate_state(|state| new_post_impl(&args, state)) {
            Ok(ok) => ok,
            Err(error) => return error,
        };

        match commit_local_index(canister_id, post_id, jwt.noble_id, &args, now).await {
            Ok(()) => {
                mutate_state(|state| {
                    state
                    .data
                    .posts
                    .add_post(
                        post_id,
                        jwt.noble_id,
                        args.title.clone(),
                        truncate_string(args.description.clone(), 100),
                        args.category.clone(),
                        args.link_url.clone(),
                        args.video_url.clone(),
                        args.attached_file_id,
                        args.post_privacy,
                        args.invited_users.clone(),
                        now,
                    );
                    state.data.local_index_map.add_post(canister_id, post_id);
                });
                Success(canister_id, post_id)
            },
            Err(err) => err,
        }
    } else {
        PermissionDenied
    }
}

fn new_post_impl(args: &Args, state: &mut RuntimeState) -> Result<(CanisterId, PostId), Response> {
    let canister_id = match prepare(args, state) {
        Ok(ok) => ok,
        Err(response) => return Err(response),
    };

    let mut post_id = state.env.rng().gen_range(1000000000u64..10000000000u64);
    while state.data.posts.get(post_id).is_some() {
        post_id = state.env.rng().gen_range(1000000000u64..10000000000u64);
    }

    Ok((canister_id, post_id))
}


fn prepare(args: &Args, state: &mut RuntimeState) -> Result<CanisterId, Response> {
    let canister_id = match state.data.local_index_map.index_for_new_post() {
        Some(index) => index,
        None => return Err(PostLimitReached),
    };

    let mut error = ErrorResult::default();

    if args.title.is_empty() {
        error.title = format!("This field is required.");
    }

    if args.title.len() > MAX_TITLE_LENGTH {
        error.description = format!("Title must be less than {} characters.", MAX_TITLE_LENGTH);
    }

    if args.description.is_empty() {
        error.description = format!("This field is required.");
    }
    
    if args.description.len() > MAX_DESCRIPTION_LENGTH {
        error.description = format!("Description must be less than {} characters.", MAX_DESCRIPTION_LENGTH);
    }

    if error.is_error() {
        return Err(Error(error));
    }

    Ok(canister_id)
}


async fn commit_local_index(
    canister_id: CanisterId,
    post_id: PostId,
    owner: NobleId,
    args: &Args,
    now: TimestampMillis
) -> Result<(), Response> {

    match local_post_index_canister_c2c_client::new_post(
        canister_id,
        &local_post_index_canister::new_post::Args {
            post_id,
            owner,
            title: args.title.clone(),
            description: args.description.clone(),
            category: args.category,
            link_url: args.link_url.clone(),
            video_url: args.video_url.clone(),
            attached_file_id: args.attached_file_id,
            post_privacy: args.post_privacy,
            invited_users: args.invited_users.clone(),
            date_created: now,
        }).await
    {
        Ok(response) => match response {
            local_post_index_canister::new_post::Response::Success => Ok(()),
            local_post_index_canister::new_post::Response::PostLimitReached => Err(PostLimitReached),
        },
        Err(error) => Err(InternalError(format!("{error:?}"))),
    }
}
