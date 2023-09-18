use ic_cdk_macros::query;
use types::{HttpRequest, HttpResponse, PostId, TimestampMillis};
use http_request::{extract_route, Route, build_json_response, encode_logs};

use crate::{read_state, RuntimeState};

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_metrics(state: &RuntimeState) -> HttpResponse {
        build_json_response(&state.metrics())
    }

    fn get_logs(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_logs(), since.unwrap_or_default())
    }

    fn get_traces(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_traces(), since.unwrap_or_default())
    }

    fn get_post(post_id: Option<PostId>, state: &RuntimeState) -> HttpResponse {
        let post_id = post_id.unwrap_or_default();

        if let Some(post) = state.data.posts.get(post_id) {
            build_json_response(&post)
        } else {
            HttpResponse::not_found()
        }
    }

    match extract_route(&request.url) {
        Route::Logs(since) => get_logs(since),
        Route::Traces(since) => get_traces(since),
        Route::Metrics => read_state(get_metrics),
        Route::Post(post_id) => read_state(|state| get_post(post_id, state)),
        _ => HttpResponse::not_found(),
    }
}