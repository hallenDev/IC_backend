use crate::RuntimeState;

pub mod sync_events_to_local_user_index_canisters;
pub mod sync_events_to_post_index_canister;
pub mod sync_events_to_send_email;
pub mod upgrade_canisters;

pub(crate) fn start(state: &RuntimeState) {
    sync_events_to_local_user_index_canisters::start_job_if_required(state);
    sync_events_to_post_index_canister::start_job_if_required(state);
    sync_events_to_send_email::start_job_if_required(state);
    upgrade_canisters::start_job_if_required(state);
}
