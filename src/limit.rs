//! Gets limit from `docker.io`'s ratelimitpreview manifest

use super::err::{DrlErr, DrlResult, ExitCode};
use super::token::Token;
use reqwest::{Client, StatusCode};

/// Gets rate limit from `docker.io`
///
/// # Arguments
///
/// `t` - `Token` JWT token from `docker.io`
///
/// # Panics
///
/// * Unexpected HTTP status codes
/// * Missing rate limit information from headers
/// * Parsing errors related to rate limit headers
pub async fn get_limit(t: &Token) -> DrlResult<String> {
    let client = Client::new();
    let url = "https://registry-1.docker.io/v2/ratelimitpreview/test/manifests/latest";
    let req = client.get(url);
    let req = req.bearer_auth(t.token.as_str());

    // send request
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("failed to connect to docker.io: {}", e);
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    // check for over limit status code
    match resp.status() {
        StatusCode::OK => (),
        StatusCode::TOO_MANY_REQUESTS => {
            let msg = String::from("over limit");
            let err = DrlErr::new(msg, ExitCode::OverLimit);
            return Err(err);
        }
        _ => {
            let msg = format!("error connecting to docker.io: {}", resp.status());
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    // limits stored in the headers
    let headers = resp.headers();

    // get rate limit
    let limit = match headers.get("ratelimit-limit") {
        Some(l) => l,
        None => {
            let msg = String::from("error parsing rate limit");
            let err = DrlErr::new(msg, ExitCode::Parsing);
            return Err(err);
        }
    };

    let limit = match limit.to_str() {
        Ok(l) => l,
        Err(e) => {
            let msg = format!("error parsing rate limit: {}", e);
            let err = DrlErr::new(msg, ExitCode::Parsing);
            return Err(err);
        }
    };

    // limit needs to be parsed from the form limit;w=window
    let mut tokens = limit.split(';');
    // There will always be at least one item in the split iterator
    let limit = tokens.next().unwrap();

    // get remaining limit
    let remaining = match headers.get("ratelimit-remaining") {
        Some(r) => r,
        None => {
            let msg = String::from("error parsing rate limit");
            let err = DrlErr::new(msg, ExitCode::Parsing);
            return Err(err);
        }
    };

    let remaining = match remaining.to_str() {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("error parsing rate limit: {}", e);
            let err = DrlErr::new(msg, ExitCode::Parsing);
            return Err(err);
        }
    };

    // remaining pulls needs to be parsed from the form remaining;w=window
    let mut tokens = remaining.split(';');
    // There will always be at least one item in the split iterator
    let remaining = tokens.next().unwrap();

    let msg = format!("{}/{}", remaining, limit);
    Ok(msg)
}
