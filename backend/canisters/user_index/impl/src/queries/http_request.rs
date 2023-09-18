use candid::Principal;
use ic_cdk_macros::query;
use types::{HttpRequest, HttpResponse, NobleId, CanisterId, TimestampMillis, AvatarId};
use http_request::{extract_route, Route, build_json_response, encode_logs};
use serde::Serialize;

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

    fn get_users(page: Option<usize>, state: &RuntimeState) -> HttpResponse {
        let page = page.unwrap_or(1);

        #[derive(Debug, Serialize)]
        struct UserInfo {
            pub noble_id: NobleId,
            pub principal: Principal,
            pub canister_id: CanisterId,
            pub username: String,
            pub first_name: String,
            pub last_name: String,
            pub email: String,
            pub date_created: TimestampMillis,
            pub avatar_id: AvatarId,
        }

        let users: Vec<UserInfo> = state.data.users.iter()
        .skip(100 * (page - 1))
        .take(100)
        .map(|user| UserInfo {
            noble_id: user.noble_id,
            principal: user.principal,
            username: user.username.clone(),
            first_name: user.first_name.clone(),
            last_name: user.last_name.clone(),
            email: user.email.clone(),
            canister_id: user.canister_id,
            date_created: user.date_created,
            avatar_id: user.avatar_id,
        }).collect();

        build_json_response(&users)
    }

    match extract_route(&request.url) {
        Route::Logs(since) => get_logs(since),
        Route::Traces(since) => get_traces(since),
        Route::Metrics => read_state(get_metrics),
        Route::Users(page) => read_state(|state| get_users(page, state)),
        _ => HttpResponse::not_found(),
    }
}