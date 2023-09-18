use crate::{mutate_state, RuntimeState, INFO_EMAIL, FEEDBACK_LIMIT};
use ic_cdk_macros::update;
use user_index_canister::{EmailEvent, Feedback};
use user_index_canister::send_feedback::{Response::*, *};

#[update]
fn send_feedback(args: Args) -> Response {
    mutate_state(|state| send_feedback_impl(&args, state))
}

fn send_feedback_impl(args: &Args, state: &mut RuntimeState) -> Response {
    if args.feedback.len() > FEEDBACK_LIMIT {
        return FeedBackTooLong(FEEDBACK_LIMIT as u32);
    }

    if !email_address::EmailAddress::is_valid(&args.email) {
        return EmailIsInvalid;
    }

    state.push_event_to_send_email(
        &args.email,
        EmailEvent::Feedback(Box::new(Feedback{ email: String::from(INFO_EMAIL), feedback: args.feedback.clone() })),
    );

    Success
}
