//! # types
//!
//! Defines the various types and aliases.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

#[cfg(feature = "encoding_rs")]
use encoding_rs::Encoding;
use fsio::error::FsIOError;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

/// Alias for result with script error
pub type ScriptResult<T> = Result<T, ScriptError>;

#[derive(Debug)]
/// Holds the error information
pub enum ScriptError {
    /// Root error
    IOError(io::Error),
    /// Root error
    FsIOError(FsIOError),
    /// Description text of the error reason
    Description(&'static str),
}

impl Display for ScriptError {
    /// Formats the value using the given formatter.
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::IOError(ref cause) => cause.fmt(format),
            Self::FsIOError(ref cause) => cause.fmt(format),
            Self::Description(description) => description.fmt(format),
        }
    }
}

impl Error for ScriptError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Description(_) => None,
            Self::IOError(error) => Some(error),
            Self::FsIOError(error) => Some(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Options available for invoking the script
pub struct ScriptOptions {
    /// Defines the requested runner (defaults to cmd in windows and sh for other platforms)
    pub runner: Option<String>,
    /// Args for the runner (for cmd, /C will automatically be added at the end)
    pub runner_args: Option<Vec<String>>,
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
    /// Environment environment variables to add before invocation
    pub env_vars: Option<std::collections::HashMap<String, String>>,
    /// Encoding conversion for stdout and stderr
    #[cfg(feature = "encoding_rs")]
    pub encoding: Option<&'static Encoding>,
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
            runner_args: None,
            working_directory: None,
            input_redirection: IoOptions::Inherit,
            output_redirection: IoOptions::Pipe,
            exit_on_error: false,
            print_commands: false,
            env_vars: None,
            #[cfg(feature = "encoding_rs")]
            encoding: None,
        }
    }
}
