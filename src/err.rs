//! Exit Codes for libdocker-rl

use std::fmt;
use std::process;

/// Exit codes
#[derive(Debug, Clone, Copy)]
pub enum ExitCode {
    /// Exit code for successful programs
    Ok = 0,
    /// Exit code when limit has been exceeded
    OverLimit,
    /// Exit code for failed authentication
    Unauthorized,
    /// Exit code for connection errors
    Connection,
    /// Error parsing body
    Body,
    /// Error parsing rate limit
    Parsing,
}

impl Default for ExitCode {
    fn default() -> Self {
        ExitCode::Ok
    }
}

/// Wrapper around result to keep track of `ExitCode`s
pub type DrlResult<T> = std::result::Result<T, DrlErr>;

impl fmt::Display for DrlErr {
    /// fmt for output
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

/// Wrapper around exit code
#[derive(Debug, Clone, Default)]
pub struct DrlErr {
    /// Message to print on exit
    pub msg: String,
    // ExitCode to use
    pub ret: ExitCode,
}

impl DrlErr {
    /// Implements constructor
    pub fn new(msg: String, ret: ExitCode) -> DrlErr {
        DrlErr { msg, ret }
    }

    /// Prints message and exits with code
    pub fn err_out(&self) -> ! {
        eprintln!("{}", &self.msg);
        process::exit(self.ret as i32);
    }
}
