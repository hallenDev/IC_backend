use ic_cdk::api::management_canister::provisional::CanisterId;
use ic_cdk_macros::query;
use types::{HttpRequest, HttpResponse, PostId, NobleId, Category, TimestampMillis};
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

    fn get_posts(page: Option<usize>, state: &RuntimeState) -> HttpResponse {
        let page = page.unwrap_or(1);

        #[derive(Debug, Serialize)]
        struct PostInfo {
            pub post_id: PostId,
            pub canister_id: CanisterId,
            pub noble_id: NobleId,
            pub title: String,
            pub description: String,
            pub category: Category,
        
            pub liked_users_count: u32,
            pub comments_count: u32,
        
            pub date_created: TimestampMillis,
            pub date_last_commented: TimestampMillis,
        }

        let posts: Vec<PostInfo> = state.data.posts.iter()
        .skip(100 * (page - 1))
        .take(100)
        .map(|post| PostInfo {
            post_id: post.post_id,
            canister_id: post.canister_id,
            noble_id: post.noble_id,
            title: post.title.clone(),
            description: post.description.clone(),
            category: post.category,
            liked_users_count: post.liked_users_count,
            comments_count: post.comments_count,
            date_created: post.date_created,
            date_last_commented: post.date_last_commented,
        }).collect();

        build_json_response(&posts)

    }

    match extract_route(&request.url) {
        Route::Logs(since) => get_logs(since),
        Route::Traces(since) => get_traces(since),
        Route::Metrics => read_state(get_metrics),
        Route::Posts(page) => read_state(|state| get_posts(page, state)),
        _ => HttpResponse::not_found(),
    }
}