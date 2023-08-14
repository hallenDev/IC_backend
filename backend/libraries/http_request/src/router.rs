use std::str::FromStr;

use types::NobleId;

pub enum Route {
    Avatar(Option<NobleId>),
    Other(String, String),
}

pub fn extract_route(path: &str) -> Route {
    let trimmed = path.trim_start_matches('/').trim_end_matches('/').to_lowercase();

    if trimmed.is_empty() {
        return Route::Other("".to_string(), "".to_string());
    }

    let (path, qs) = trimmed.split_once('?').unwrap_or((&trimmed, ""));

    let parts: Vec<_> = path.split('/').collect();

    match parts[0] {
        "avatar" => {
            let blob_id = parts.get(1).and_then(|p| NobleId::from_str(p).ok());
            return Route::Avatar(blob_id);
        }
        _ => (),
    }

    Route::Other(path.to_string(), qs.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn avatar() {
        const BLOB_ID: NobleId = 3672535213;
        match extract_route(&format!("/avatar/{BLOB_ID}")) {
            Route::Avatar(Some(id)) => assert_eq!(BLOB_ID, id),
            _ => panic!(),
        }
    }

    #[test]
    fn other() {
        assert!(matches!(extract_route("blah"), Route::Other(_, _)));
    }

    #[test]
    fn querystring() {
        let route = extract_route("blah?abc=1");
        if let Route::Other(p, qs) = route {
            assert_eq!(&p, "blah");
            assert_eq!(&qs, "abc=1");
        } else {
            panic!();
        }
    }
}
