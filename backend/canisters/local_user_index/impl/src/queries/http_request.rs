use ic_cdk_macros::query;
use types::{HttpRequest, HttpResponse, NobleId, TimestampMillis, AvatarId};
use http_request::{extract_route, Route, build_response, build_json_response, encode_logs};

use crate::{read_state, RuntimeState};

#[query]
fn http_request(request: HttpRequest) -> HttpResponse {
    fn get_avatar(avatar_id: Option<AvatarId>, state: &RuntimeState) -> HttpResponse {
        if avatar_id.is_none() {
            return HttpResponse::not_found();
        }
        if let Some(noble_id) = state.data.users.avatar_id_to_noble_id.get(&avatar_id.unwrap()) {
            if let Some(user) = state.data.users.get(*noble_id) {
                return build_response(user.photo.clone(), "image/jpeg");
            }
        }
        HttpResponse::not_found()
    }

    fn get_metrics(state: &RuntimeState) -> HttpResponse {
        build_json_response(&state.metrics())
    }

    fn get_logs(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_logs(), since.unwrap_or_default())
    }

    fn get_traces(since: Option<TimestampMillis>) -> HttpResponse {
        encode_logs(canister_logger::export_traces(), since.unwrap_or_default())
    }

    fn get_user(noble_id: Option<NobleId>, state: &RuntimeState) -> HttpResponse {
        let noble_id = noble_id.unwrap_or_default();

        if let Some(user) = state.data.users.get(noble_id) {
            build_json_response(&user)
        } else {
            HttpResponse::not_found()
        }
    }

    match extract_route(&request.url) {
        Route::Avatar(noble_id) => read_state(|state| get_avatar(noble_id, state)),
        Route::Logs(since) => get_logs(since),
        Route::Traces(since) => get_traces(since),
        Route::Metrics => read_state(get_metrics),
        Route::User(user_id) => read_state(|state| get_user(user_id, state)),
        _ => HttpResponse::not_found(),
    }
}