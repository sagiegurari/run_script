//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::error;
use std::fmt;
use std::fmt::Display;
use std::io::Error;

#[derive(Debug)]
/// Holds the error information
pub enum ErrorInfo {
    /// Root error
    IOError(Error),
    /// Description text of the error reason
    Description(&'static str),
}

#[derive(Debug)]
/// Error struct
pub struct ScriptError {
    /// Holds the error information
    pub info: ErrorInfo,
}

impl error::Error for ScriptError {
    /// A short description of the error.
    fn description(&self) -> &str {
        match self.info {
            ErrorInfo::IOError(ref cause) => cause.description(),
            ErrorInfo::Description(description) => description,
        }
    }

    /// The lower-level cause of this error, if any.
    fn cause(&self) -> Option<&error::Error> {
        match self.info {
            ErrorInfo::IOError(ref cause) => Some(cause),
            _ => None,
        }
    }
}

impl Display for ScriptError {
    /// Formats the value using the given formatter.
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.info {
            ErrorInfo::IOError(ref cause) => cause.fmt(format),
            ErrorInfo::Description(description) => description.fmt(format),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Options available for invoking the script
pub struct ScriptOptions {
    /// Defines the requested runner (defaults to cmd in windows and sh for other platforms)
    pub runner: Option<String>,
    /// False to print the output to the parent process, or capture and return the output (default)
    pub capture_output: bool,
    /// Sets -e flag. Will exit on any error while running the script (not available for windows)
    pub exit_on_error: bool,
    /// Sets -x flag for printing each script command before invocation (not available for windows)
    pub print_commands: bool,
}

impl ScriptOptions {
    /// Returns new instance
    pub fn new() -> ScriptOptions {
        ScriptOptions {
            runner: None,
            capture_output: true,
            exit_on_error: false,
            print_commands: false,
        }
    }
}
