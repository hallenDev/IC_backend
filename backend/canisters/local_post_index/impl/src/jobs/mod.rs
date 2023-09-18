use crate::RuntimeState;

pub mod sync_events_to_post_index_canister;
pub mod sync_events_to_user_index_canister;

pub(crate) fn start(state: &RuntimeState) {
    sync_events_to_post_index_canister::start_job_if_required(state);
    sync_events_to_user_index_canister::start_job_if_required(state);
}
