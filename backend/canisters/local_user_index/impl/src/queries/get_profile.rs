use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::get_profile::{Response::*, *};
use types::check_jwt;

#[query]
fn get_profile(args: Args) -> Response {
    read_state(|state| get_profile_impl(args, state))
}

fn get_profile_impl(args: Args, state: &RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            Success(SuccessResult{
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                gender: user.gender,
                degree: user.degree,
                bio: user.bio.clone(),
                country: user.country,
                city: user.city.clone(),
                preferred_pronouns: user.preferred_pronouns.clone(),
                linkedin_handle: user.linkedin_handle.clone(),
                twitter_handle: user.twitter_handle.clone(),
                mastodon_handle: user.mastodon_handle.clone(),
                github_handle: user.github_handle.clone(),
                facebook_handle: user.facebook_handle.clone(),
                personal_website: user.personal_website.clone(),
            })
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}
