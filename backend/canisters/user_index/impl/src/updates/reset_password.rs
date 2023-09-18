use crate::{mutate_state, RuntimeState, INFO_EMAIL};
use crate::model::temp::{TempData, ResetPassword};
use ic_cdk_macros::update;
use rand::{Rng, distributions::Alphanumeric};
use user_index_canister::{EmailEvent, ResetPasswordVerify};
use user_index_canister::reset_password::{Response::*, *};

#[update]
fn reset_password(args: Args) -> Response {
    mutate_state(|state| reset_password_impl(&args, state))
}

fn reset_password_impl(args: &Args, state: &mut RuntimeState) -> Response {
    state.data.temps.remove_expired_temp(state.env.now());

    if let Some(user) = state.data.users.get_by_email(&args.email) {
        if user.email.is_empty() {
            return EmailNotSet;
        }
        let password: String = state.env.rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect();
        let now = state.env.now();
        let (temp_id, passkey) = state.data.temps.add_new_temp(
            args.email.clone(),
            TempData::ResetPassword(ResetPassword { name: user.username.clone(), password: password.clone() }), state.env.rng(), now
        );
        state.push_event_to_send_email(
            INFO_EMAIL,
            EmailEvent::ResetPasswordVerify(Box::new(ResetPasswordVerify { email: user.email.clone(), name: user.username.clone(), passkey }))
        );
        Success(temp_id)
    } else {
        UserNotFound
    }
}
