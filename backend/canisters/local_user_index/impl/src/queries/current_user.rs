use crate::{read_state, RuntimeState};
use ic_cdk_macros::query;
use local_user_index_canister::current_user::{Response::*, *};
use types::check_jwt;

#[query]
fn current_user(args: Args) -> Response {
    read_state(|state| current_user_impl(args, state))
}

fn current_user_impl(args: Args, state: &RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            Success(SuccessResult{
                noble_id: user.noble_id,
                username: user.username.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                country: user.country.clone(),
                city: user.city.clone(),
            
                preferred_pronouns: user.preferred_pronouns.clone(),
                photo: user.photo.clone(),
                email: user.email.clone(),
                search_by_email: user.search_by_email,
                bio: user.bio.clone(),
            
                account_privacy: user.account_privacy,
    
                linkedin_handle: user.linkedin_handle.clone(),
                twitter_handle: user.twitter_handle.clone(),
                mastodon_handle: user.mastodon_handle.clone(),
                github_handle: user.github_handle.clone(),
                facebook_handle: user.facebook_handle.clone(),
                personal_website: user.personal_website.clone(),
    
                followers: user.followers.clone(),
                following_list: user.following_list.clone(),
                block_users: user.block_users.clone(),
    
                date_created: user.date_created,
                date_updated: user.date_updated,
            })
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}
