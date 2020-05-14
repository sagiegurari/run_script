//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use fsio::error::FsIOError;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug)]
/// Holds the error information
pub enum ErrorInfo {
    /// Root error
    IOError(Error),
    /// Root error
    FsIOError(FsIOError),
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
    /// The lower-level cause of this error, if any.
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
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
            ErrorInfo::FsIOError(ref cause) => cause.fmt(format),
            ErrorInfo::Description(description) => description.fmt(format),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Options available for invoking the script
pub struct ScriptOptions {
    /// Defines the requested runner (defaults to cmd in windows and sh for other platforms)
    pub runner: Option<String>,
    /// The working directory of the invocation
    pub working_directory: Option<PathBuf>,
    /// Default is IoOptions::Inherit
    pub input_redirection: IoOptions,
    /// Default is IoOptions::Pipe (only pipe enables to capture the output)
    pub output_redirection: IoOptions,
    /// Sets -e flag. Will exit on any error while running the script (not available for windows)
    pub exit_on_error: bool,
    /// Sets -x flag for printing each script command before invocation (not available for windows)
    pub print_commands: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// Options available for IO
pub enum IoOptions {
    /// Corresponds to Stdio::null()
    Null,
    /// Corresponds to Stdio::pipe()
    Pipe,
    /// Corresponds to Stdio::inherit()
    Inherit,
}

impl ScriptOptions {
    /// Returns new instance
    pub fn new() -> ScriptOptions {
        ScriptOptions {
            runner: None,
            working_directory: None,
            input_redirection: IoOptions::Inherit,
            output_redirection: IoOptions::Pipe,
            exit_on_error: false,
            print_commands: false,
        }
    }
}
