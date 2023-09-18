use crate::{mutate_state, RuntimeState, MAX_BIO_LENGTH};
use ic_cdk_macros::update;
use local_user_index_canister::set_profile::{Response::*, *};
use user_index_canister::{Event as UserIndexEvent, ProfileChanged};
use types::check_jwt;
use utils::{truncate_string::truncate_string, field_validation::validate_field_value};

#[update]
fn set_profile(args: Args) -> Response {
    mutate_state(|state| set_profile_impl(args, state))
}

fn set_profile_impl(args: Args, state: &mut RuntimeState) -> Response {
    if let Some(jwt) = check_jwt(&args.jwt, state.env.now()) {
        match prepare(&args) {
            Ok(()) => {},
            Err(error) => return error,
        }

        if let Some(user) = state.data.users.get_mut(jwt.noble_id) {
            user.first_name = args.first_name.clone();
            user.last_name = args.last_name.clone();
            user.gender = args.gender;
            user.degree = args.degree;
            user.bio = args.bio.clone();
            user.personal_website = args.personal_website;
            user.country = args.country;
            user.city = args.city.clone();
            user.preferred_pronouns = args.preferred_pronouns;
            user.linkedin_handle = args.linkedin_handle;
            user.twitter_handle = args.twitter_handle;
            user.mastodon_handle = args.mastodon_handle;
            user.github_handle = args.github_handle;
            user.facebook_handle = args.facebook_handle;

            let avatar_id = user.avatar_id;

            user.date_updated = state.env.now();

            state.push_event_to_user_index(UserIndexEvent::ProfileChanged(Box::new(
                ProfileChanged {
                    noble_id: jwt.noble_id,
                    first_name: args.first_name,
                    last_name: args.last_name,
                    country: args.country,
                    city: args.city,
                    degree: args.degree,
                    bio: truncate_string(args.bio, 100),
                    avatar_id,
                }
            )));
            Success(avatar_id)
        } else {
            UserNotFound
        }
    } else {
        PermissionDenied
    }
}

fn prepare(args: &Args) -> Result<(), Response> {
    let mut error = ErrorResult::new();

    if let Err(err) = validate_field_value("First name", true, 50, &args.first_name, utils::field_validation::FieldType::Text) {
        error.first_name = err;
    }

    if let Err(err) = validate_field_value("Last name", true, 50, &args.last_name, utils::field_validation::FieldType::Text) {
        error.last_name = err;
    }

    if let Err(err) = validate_field_value("Bio", true, MAX_BIO_LENGTH, &args.bio, utils::field_validation::FieldType::Text) {
        error.bio = err;
    }

    if let Err(err) = validate_field_value("City", true, 50, &args.city, utils::field_validation::FieldType::Text) {
        error.city = err;
    }

    if let Err(err) = validate_field_value("Url", false, 50, &args.personal_website, utils::field_validation::FieldType::Url) {
        error.personal_website = err;
    }

    if let Err(err) = validate_field_value("Url", false, 50, &args.linkedin_handle, utils::field_validation::FieldType::Url) {
        error.linkedin_handle = err;
    }

    if let Err(err) = validate_field_value("Url", false, 50, &args.twitter_handle, utils::field_validation::FieldType::Url) {
        error.twitter_handle = err;
    }

    if let Err(err) = validate_field_value("Url", false, 50, &args.mastodon_handle, utils::field_validation::FieldType::Url) {
        error.mastodon_handle = err;
    }

    if let Err(err) = validate_field_value("Url", false, 50, &args.github_handle, utils::field_validation::FieldType::Url) {
        error.github_handle = err;
    }

    if let Err(err) = validate_field_value("Url", false, 50, &args.facebook_handle, utils::field_validation::FieldType::Url) {
        error.facebook_handle = err;
    }

    if args.country.is_none() {
        error.country = format!("Country should be selected");
    }

    if args.degree.is_none() {
        error.degree = format!("Degree should be selected");
    }

    if args.preferred_pronouns.is_none() {
        error.preferred_pronouns = format!("Preferred pronouns should be selected");
    }

    if error.is_error() {
        Err(Error(error))
    } else {
        Ok(())
    }
}
