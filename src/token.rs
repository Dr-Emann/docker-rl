//! Module to get JWT tokens from `docker.io`
//!
//! Supports usr/pass with basic authentication

use super::err::{DrlErr, DrlResult, ExitCode};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json;

/// Struct to hold token information
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Token {
    pub token: String,
    pub expires_in: usize,
    pub issued_at: String,
}

impl Token {
    /// Creates an empty token
    pub fn new() -> Token {
        Token::default()
    }
}

const DOCKER_URL: &str = "https://auth.docker.io/token?service=registry.docker.io&scope=repository:ratelimitpreview/test:pull";

/// Get anonymous token from `docker.io`
///
/// Returns `Token` with JWT token info
pub async fn get_anon_token() -> DrlResult<Token> {
    let client = Client::new();
    let req = client.get(DOCKER_URL);

    // send request
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("failed to connect to docker.io: {}", e);
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    // check status for errors
    match resp.status() {
        StatusCode::OK => (),
        _ => {
            let msg = format!("unknown response {:?}", resp.status());
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    let body = match resp.text().await {
        Ok(b) => b,
        Err(e) => {
            let msg = format!("failed to parse response: {}", e);
            let err = DrlErr::new(msg, ExitCode::Body);
            return Err(err);
        }
    };

    // unmarshal
    let t: Token = match serde_json::from_str(body.as_str()) {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("failed to parse response: {}", e);
            let err = DrlErr::new(msg, ExitCode::Body);
            return Err(err);
        }
    };

    Ok(t)
}

/// Get token from `docker.io` with user/pass
///
/// Returns `Token` with JWT token info
///
/// # Arguments
///
/// * `user` - `String` with username
/// * `pass` - `String` with passphrase
///
pub async fn get_userpass_token(user: String, pass: String) -> DrlResult<Token> {
    let client = Client::new();
    let req = client.get(DOCKER_URL);
    let req = req.basic_auth(&user, Some(pass));

    // actually send request
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            let msg = format!("failed to connect to docker.io: {}", e);
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    // check status for auth errors
    match resp.status() {
        StatusCode::OK => (),
        StatusCode::UNAUTHORIZED => {
            let msg = format!("authentication failed for {}", &user);
            let err = DrlErr::new(msg, ExitCode::Unauthorized);
            return Err(err);
        }
        _ => {
            let msg = format!("unknown response {:?}", resp.status());
            let err = DrlErr::new(msg, ExitCode::Connection);
            return Err(err);
        }
    };

    let body = match resp.text().await {
        Ok(b) => b,
        Err(e) => {
            let msg = format!("failed to parse response: {}", e);
            let err = DrlErr::new(msg, ExitCode::Body);
            return Err(err);
        }
    };

    let t: Token = match serde_json::from_str(body.as_str()) {
        Ok(t) => t,
        Err(e) => {
            let msg = format!("failed to parse response: {}", e);
            let err = DrlErr::new(msg, ExitCode::Body);
            return Err(err);
        }
    };

    Ok(t)
}
