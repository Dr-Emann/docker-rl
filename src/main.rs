//! #Docker-RL
//!
//! Command line utility to check docker rate limit
//!
//! **Note:** check the rate limit lowers the limit
//!
//! **Note:** docker reports the limit before decrementing it
//! so it's 1 less.
//!
//! # Examples
//!
//! # Anonymous Rate Limit
//!
//! ```sh
//!  > docker-rl
//!  > 97/100
//! ```
//!
//! # User
//! ```sh
//!  > docker-rl -u someuser
//!  > Password for someuser:
//!  > 97/200
//! ```
//!
//! # User/Pass
//! ```sh
//!  > docker-rl -u someuser -p somepass
//!  > 97/200
//! ```

use libdocker_rl::err::DrlResult;
use libdocker_rl::limit::get_limit;
use libdocker_rl::options::Opts;
use libdocker_rl::token::{get_anon_token, get_userpass_token, Token};
use rpassword::read_password_from_tty;

/// Parses options stuct and gets jwt token
///
/// # Arguments
///
/// * `opts` - `Opts` struct with parsed options
async fn get_token(opts: Opts) -> DrlResult<Token> {
    let Opts { user, pass } = opts;

    if let Some(user) = user {
        let pass = pass.unwrap_or_else(|| {
            // rpassword docs say:
            //   Prompt for a password on TTY (safest but not always most practical
            //   when integrating with other tools or unit testing)
            //
            // should this have error handling?

            let prompt = format!("Password for {}: ", user);
            read_password_from_tty(Some(&prompt)).unwrap()
        });

        get_userpass_token(user, pass).await
    } else {
        get_anon_token().await
    }
}

/// Parses cmdline and prints rate limit
#[tokio::main]
async fn main() {
    // parse arguments
    let opts = Opts::parse_args();

    // get auth token for docker hub
    let result = get_token(opts).await;
    let token = result.unwrap_or_else(|e| e.err_out());

    // get limit from token
    let result = get_limit(&token).await;
    let limit = result.unwrap_or_else(|e| e.err_out());

    println!("{}", limit);
}
