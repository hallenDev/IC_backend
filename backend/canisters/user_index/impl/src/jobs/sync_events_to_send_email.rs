use crate::{mutate_state, RuntimeState};
use ic_cdk::api::management_canister::http_request::{CanisterHttpRequestArgument, HttpMethod, HttpHeader, http_request};

use ic_cdk_timers::TimerId;
use crate::EmailEvent;
use std::cell::Cell;
use std::time::Duration;
use tracing::{trace, info};

thread_local! {
    static TIMER_ID: Cell<Option<TimerId>> = Cell::default();
}

#[allow(dead_code)]
pub(crate) fn start_job_if_required(state: &RuntimeState) -> bool {
    if TIMER_ID.with(|t| t.get().is_none()) && !state.data.email_event_sync_queue.is_empty() {
        let timer_id = ic_cdk_timers::set_timer_interval(Duration::ZERO, run);
        TIMER_ID.with(|t| t.set(Some(timer_id)));
        trace!("'sync_events_to_send_email_canisters' job started");
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
                trace!("'sync_events_to_send_email_canisters' job stopped");
            }
        }
    }
}

enum GetNextResult {
    Success(Vec<(String, Vec<EmailEvent>)>),
    Continue,
    QueueEmpty,
}

fn try_get_next(state: &mut RuntimeState) -> GetNextResult {
    if state.data.email_event_sync_queue.is_empty() {
        GetNextResult::QueueEmpty
    } else if let Some(batch) = state.data.email_event_sync_queue.try_start_batch() {
        GetNextResult::Success(batch)
    } else {
        GetNextResult::Continue
    }
}

async fn process_batch(batch: Vec<(String, Vec<EmailEvent>)>) {
    let futures: Vec<_> = batch
        .into_iter()
        .map(|(email, events)| sync_events(email, events))
        .collect();

    futures::future::join_all(futures).await;

    mutate_state(|state| state.data.email_event_sync_queue.mark_batch_completed());
}

async fn sync_events(email: String, events: Vec<EmailEvent>) {
    for event in events {
        send_msg(&email, event).await;
    }
}

async fn send_msg(email: &str, event: EmailEvent) {
    let host = "api.mailgun.net";
    let domain = "nobleblocks.com";
    let url = format!("https://{host}/v3/{domain}/messages");

    let request_headers = vec![
        HttpHeader {
            name: "Host".to_string(),
            value: format!("{host}"),
        },
        HttpHeader {
            name: "User-Agent".to_string(),
            value: "NobleBlocks".to_string(),
        },
        HttpHeader {
            name: "Authorization".to_string(),
            value: "Basic YXBpOjFhN2NlMDE3NDI2ZWU5NzcwM2M2MzY4ODhhOWUwMmI1LWYwZTUwYTQyLTQ3MzA3NzMz".to_string(),
        },
        HttpHeader {
            name: "Content-Type".to_string(),
            value: "multipart/form-data; boundary=___NOBLEBLOCKS_BOUNDARY___".to_string(),
        },
        HttpHeader {
            name: "Accept".to_string(),
            value: "*/*".to_string(),
        },
        HttpHeader {
            name: "Accept-Encoding".to_string(),
            value: "gzip, deflate, br".to_string(),
        },
        HttpHeader {
            name: "Connection".to_string(),
            value: "keep-alive".to_string(),
        },
    ];

    let (to, subject, content) = get_content(email, event);

    let json_string: String = format!(r#"
--___NOBLEBLOCKS_BOUNDARY___
Content-Disposition: form-data; name="from"

{email}
--___NOBLEBLOCKS_BOUNDARY___
Content-Disposition: form-data; name="to"

{to}
--___NOBLEBLOCKS_BOUNDARY___
Content-Disposition: form-data; name="subject"

{subject}
--___NOBLEBLOCKS_BOUNDARY___
Content-Disposition: form-data; name="html"

{content}
--___NOBLEBLOCKS_BOUNDARY___--
"#);

    let json_utf8: Vec<u8> = json_string.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: url.to_string(),
        max_response_bytes: Some(300), //optional for request
        method: HttpMethod::POST,
        headers: request_headers,
        body: request_body,
        transform: None, //optional for request
    };

    let cost = get_cost(&request);

    match http_request(request, cost).await {
        Ok(_) => info!("success", ),
        Err(err) => info!("error: {:?}", err),
    }
}

fn get_cost(arg: &CanisterHttpRequestArgument) -> u128 {
    let max_response_bytes = match arg.max_response_bytes {
        Some(ref n) => *n as u128,
        None => 2 * 1024 * 1024u128, // default 2MiB
    };
    let arg_raw = candid::encode_args((arg,)).expect("Failed to encode arguments.");
    // The coefficients can be found in [this page](https://internetcomputer.org/docs/current/developer-docs/production/computation-and-storage-costs).
    // 12 is "http_request".len().
    let n = 13;
    let per_call_cost = (3_000_000u128 + 60_000u128 * n) * n;
    let per_request_byte_cost = 400u128 * n;
    let per_response_byte_cost = 800u128 * n;

    per_call_cost + per_request_byte_cost * (arg_raw.len() as u128 + 12 - 182) + per_response_byte_cost * max_response_bytes
}

fn get_content(_email: &str, event: EmailEvent) -> (String, String, String) {
    match event {
        EmailEvent::RegisterUser(data) => {
            (data.email, format!("Request To Verify Your Email Address(nobleblock.com)"), format!(r#"
<div style="width: 100%; padding: 10 auto;">
    <div style="max-width: 1000px;">
        <div style="font-size: 40px; font-weight: bold;display: flex; justify-content: center; max-width: 1600px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;padding-top: 30px;">
            <span style="font-size: 40;">Request To Verify Your Email Address</span>
        </div>
        <div style="width: 100%; margin: 30px;">
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Hi, {}</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">A request to verify your email address has been detected on NOBLEBLOCKS. To proceed, please</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">enter the verification code provided below:</p>
            <p style="font-size: 20px;font-weight: bold;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;">Verification Code: {}</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">For your security, this code will expire in 3 minutes and is valid for only one use.</p>
            <br />
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Best regards</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">NOBLEBLOCKS Team</p>
            <a href="https://nobleblocks.com" style="margin-top: 20px; color: #1155cc">www.nobleblocks.com</a>
            <p style="background: #888888; width: 100%; height: 2px;"></p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;"><i>Please do not reply to this email as it is automatically generated.</i></p>
            <p style="background: #888888; width: 100%; height: 2px;"></p>
        </div>
    </div>
</div>"#, data.name, data.passkey))
        },
        EmailEvent::ResetPasswordVerify(data) => {
            (data.email, format!("Request To Reset Your NOBLEBLOCKS Password(nobleblock.com)"), format!(r#"
<div style="width: 100%; padding: 10 auto;">
    <div style="max-width: 1000px;">
        <div style="font-size: 40px; font-weight: bold;display: flex; justify-content: center; max-width: 1600px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;padding-top: 30px;">
            <span style="font-size: 40;">Request To Reset Your NOBLEBLOCKS Password</span>
        </div>
        <div style="width: 100%; margin: 30px;">
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Hi, {}</a></p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">A request to reset your password has been detected on NOBLEBLOCKS. To proceed, please</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Enter the verification code provided below:</p>
            <p style="font-size: 20px;font-weight: bold;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;">Verification Code: {}</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">For your security, this code will expire in 3 minutes and is valid for only one use.</p>
            <br />
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Best regards</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">NOBLEBLOCKS Team</p>
            <a href="https://nobleblocks.com" style="margin-top: 20px; color: #1155cc">www.nobleblocks.com</a>
            <p style="background: #888888; width: 100%; height: 2px;"></p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;"><i>Please do not reply to this email as it is automatically generated.</i></p>
            <p style="background: #888888; width: 100%; height: 2px;"></p>
        </div>
    </div>
</div>"#, data.name, data.passkey))
        },
        EmailEvent::ResetPassword(data) => {
            (data.email, format!("Your password has been successfully reset(nobleblock.com)"), format!(r#"
<div style="width: 100%; padding: 10 auto;">
    <div style="max-width: 1000px;">
        <div style="font-size: 40px; font-weight: bold;display: flex; justify-content: center; max-width: 1600px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;padding-top: 30px;">
            <span style="font-size: 40;">Your password has been successfully reset</span>
        </div>
        <div style="width: 100%; margin: 30px;">
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Hi, {}</a></p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Your password has been successfully reset.</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Please use following password from now:</p>
            <p style="font-size: 20px;font-weight: bold;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;">New password: {}</p>
            <br />
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">Best regards</p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;">NOBLEBLOCKS Team</p>
            <a href="https://nobleblocks.com" style="margin-top: 20px; color: #1155cc">www.nobleblocks.com</a>
            <p style="background: #888888; width: 100%; height: 2px;"></p>
            <p style="font-size: 16px;font-family: Arial,'Helvetica Neue',Helvetica,sans-serif;color: black;"><i>Please do not reply to this email as it is automatically generated.</i></p>
            <p style="background: #888888; width: 100%; height: 2px;"></p>
        </div>
    </div>
</div>"#, data.name, data.password))
        },
        EmailEvent::Feedback(data) => {
            (data.email, format!("User feedback"), data.feedback)
        }
    }
}
