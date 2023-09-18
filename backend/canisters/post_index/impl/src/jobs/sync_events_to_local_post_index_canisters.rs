use crate::{mutate_state, RuntimeState};
use ic_cdk_timers::TimerId;
use local_post_index_canister::Event as LocalPostIndexEvent;
use std::cell::Cell;
use std::time::Duration;
use tracing::trace;
use types::CanisterId;

thread_local! {
    static TIMER_ID: Cell<Option<TimerId>> = Cell::default();
}

pub(crate) fn start_job_if_required(state: &RuntimeState) -> bool {
    if TIMER_ID.with(|t| t.get().is_none()) && !state.data.post_index_event_sync_queue.is_empty() {
        let timer_id = ic_cdk_timers::set_timer_interval(Duration::ZERO, run);
        TIMER_ID.with(|t| t.set(Some(timer_id)));
        trace!("'sync_events_to_local_post_index_canisters' job started");
        true
    } else {
        false
    }
}

pub fn run() {
    match mutate_state(try_get_next) {
        GetNextResult::Success(batch) => {
            ic_cdk::spawn(process_batch(batch));
        }
        GetNextResult::Continue => {}
        GetNextResult::QueueEmpty => {
            if let Some(timer_id) = TIMER_ID.with(|t| t.take()) {
                ic_cdk_timers::clear_timer(timer_id);
                trace!("'sync_events_to_local_post_index_canisters' job stopped");
            }
        }
    }
}

enum GetNextResult {
    Success(Vec<(CanisterId, Vec<LocalPostIndexEvent>)>),
    Continue,
    QueueEmpty,
}

fn try_get_next(state: &mut RuntimeState) -> GetNextResult {
    if state.data.post_index_event_sync_queue.is_empty() {
        GetNextResult::QueueEmpty
    } else if let Some(batch) = state.data.post_index_event_sync_queue.try_start_batch() {
        GetNextResult::Success(batch)
    } else {
        GetNextResult::Continue
    }
}

async fn process_batch(batch: Vec<(CanisterId, Vec<LocalPostIndexEvent>)>) {
    let futures: Vec<_> = batch
        .into_iter()
        .map(|(canister_id, events)| sync_events(canister_id, events))
        .collect();

    futures::future::join_all(futures).await;

    mutate_state(|state| state.data.post_index_event_sync_queue.mark_batch_completed());
}

async fn sync_events(canister_id: CanisterId, events: Vec<LocalPostIndexEvent>) {
    let args = local_post_index_canister::c2c_notify_events::Args { events: events.clone() };
    if local_post_index_canister_c2c_client::c2c_notify_events(canister_id, &args)
        .await
        .is_err()
    {
        mutate_state(|state| {
            state
                .data
                .post_index_event_sync_queue
                .mark_sync_failed_for_canister(canister_id, events);

            start_job_if_required(state);
        });
    }
}
