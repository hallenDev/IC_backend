use crate::model::user_map::UpdateUserResult;
use crate::{mutate_state, RuntimeState};
use ic_cdk_macros::update;
use types::check_jwt;
use url::Url;
use local_user_index_canister::set_social_links::{Response::*, *};

#[update]
fn set_social_links(args: Args) -> Response {
    mutate_state(|state| set_social_links_impl(args, state))
}

fn set_social_links_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        if let Err(response) = prepare(&args, state) {
            return response;
        }
    
        if let Some(user) = state.data.users.get(jwt.noble_id) {
            let mut user_to_update = user.clone();
    
            user_to_update.linkedin_handle = args.linkedin_handle;
            user_to_update.twitter_handle = args.twitter_handle;
            user_to_update.mastodon_handle = args.mastodon_handle;
            user_to_update.github_handle = args.github_handle;
            user_to_update.facebook_handle = args.facebook_handle;
            user_to_update.personal_website = args.personal_website;
    
            let now = state.env.now();
    
            match state.data.users.update(user_to_update, now) {
                UpdateUserResult::Success => Success,
                UpdateUserResult::UserNotFound => UserNotFound,
            }
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}

fn prepare(args: &Args, _state: &mut RuntimeState) -> Result<(), Response> {
    let mut error = ErrorResult::new();

    if !args.linkedin_handle.is_empty() && Url::parse(&args.linkedin_handle).is_err() {
        error.linkedin_handle = "Invalid Url".to_string();
    }

    if !args.twitter_handle.is_empty() && Url::parse(&args.twitter_handle).is_err() {
        error.twitter_handle = "Invalid Url".to_string();
    }

    if !args.mastodon_handle.is_empty() && Url::parse(&args.mastodon_handle).is_err() {
        error.mastodon_handle = "Invalid Url".to_string();
    }

    if !args.github_handle.is_empty() && Url::parse(&args.github_handle).is_err() {
        error.github_handle = "Invalid Url".to_string();
    }

    if !args.facebook_handle.is_empty() && Url::parse(&args.facebook_handle).is_err() {
        error.facebook_handle = "Invalid Url".to_string();
    }

    if !args.personal_website.is_empty() && Url::parse(&args.personal_website).is_err() {
        error.personal_website = "Invalid Url".to_string();
    }

    if error.is_error() {
        return Err(Error(error));
    }

    Ok(())
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

        let jwt = JWT::new(1, "".to_string(), "".to_string(), runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            github_handle: "https://github.com/Wonder0729".to_string(),
            ..Default::default()
        };
        let result = set_social_links_impl(args, &mut runtime_state);
        assert_eq!(result, Response::Success);
    }

    #[test]
    fn invalid_url() {
        let mut runtime_state = setup_runtime_state();

        let jwt = JWT::new(1, "".to_string(), "".to_string(), runtime_state.env.now());
        let args = Args {
            jwt: jwt.to_string().unwrap(),
            github_handle: "https//github.com/Wonder0729".to_string(),
            ..Default::default()
        };
        let result = set_social_links_impl(args, &mut runtime_state);
        assert_eq!(result, Response::Error(ErrorResult{ github_handle: "Invalid Url".to_string(), ..Default::default() }));
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
