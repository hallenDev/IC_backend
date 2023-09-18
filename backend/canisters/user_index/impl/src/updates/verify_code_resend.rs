use crate::{mutate_state, RuntimeState, model::{temp::TempData, temp_map::{TEMP_EXPIRED_DURATION, AVAILABLE_RESEND_DURATION}}, INFO_EMAIL};
use ic_cdk_macros::update;
use user_index_canister::{verify_code_resend::{Response::*, *}, EmailEvent};

#[update]
fn verify_code_resend(args: Args) -> Response {
    mutate_state(|state| verify_code_resend_impl(&args, state))
}

fn verify_code_resend_impl(args: &Args, state: &mut RuntimeState) -> Response {
    let now = state.env.now();
    state.data.temps.remove_expired_temp(now);

    let passkey = state.data.temps.new_passkey(state.env.rng());
    
    if let Some(temp) = state.data.temps.get_mut(args.id) {
        if temp.email != args.email {
            return EmailNotCorrect;
        }
        
        if temp.is_used == false && temp.expired_time - TEMP_EXPIRED_DURATION + AVAILABLE_RESEND_DURATION > now {
            return AlreadySent;
        }
        let email = temp.email.clone();

        temp.expired_time = now + TEMP_EXPIRED_DURATION;
        temp.is_used = false;
        temp.passkey = passkey.clone();

        match &temp.temp_data {
            TempData::RegisterUser(data) => {
                let name = data.username.clone();
                state.push_event_to_send_email(
                    INFO_EMAIL,
                    EmailEvent::RegisterUser(Box::new(user_index_canister::RegisterUser { email, name, passkey }))
                );
            },
            TempData::ResetPassword(data) => {
                let name = data.name.clone();
                state.push_event_to_send_email(
                    INFO_EMAIL,
                    EmailEvent::ResetPasswordVerify(Box::new(user_index_canister::ResetPasswordVerify { email, name, passkey }))
                );
            }
        };

        return Success;
    }
    TempNotExist
}
