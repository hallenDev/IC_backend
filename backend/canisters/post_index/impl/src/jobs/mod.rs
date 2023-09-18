use crate::RuntimeState;

pub mod upgrade_canisters;
pub mod sync_events_to_local_post_index_canisters;
pub mod sync_events_to_user_index_canister;

pub(crate) fn start(state: &RuntimeState) {
    upgrade_canisters::start_job_if_required(state);
    sync_events_to_local_post_index_canisters::start_job_if_required(state);
    sync_events_to_user_index_canister::start_job_if_required(state);
}
