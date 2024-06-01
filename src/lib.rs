#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # run_script
//!
//! Run shell scripts in [rust](https://www.rust-lang.org/).
//!
//! This library enables to invoke shell scripts based on their content.<br>
//! While std::process::Command works great to execute standalone command, you need more manual code to take a script
//! text and execute it.<br>
//! For this purpose, this library was created.
//!
//! # Examples
//!
//! ## Basic Example
//!
//! ````use run_script;
//! use run_script::ScriptOptions;
//!
//! fn main() {
//!     let options = ScriptOptions::new();
//!
//!     let args = vec![];
//!
//!     // run the script and get the script execution output
//!     let (code, output, error) = run_script::run(
//!         r#"
//!          echo "Directory Info:"
//!          dir
//!          "#,
//!         &args,
//!         &options,
//!     )
//!     .unwrap();
//!
//!     println!("Exit Code: {}", code);
//!     println!("Output: {}", output);
//!     println!("Error: {}", error);
//!
//!     // run the script and get a handle to the running child process
//!     let child = run_script::spawn(
//!         r#"
//!          echo "Directory Info:"
//!          dir
//!          "#,
//!         &args,
//!         &options,
//!     )
//!     .unwrap();
//!
//!     let spawn_output = child.wait_with_output().unwrap();
//!
//!     println!("Success: {}", &spawn_output.status.success());
//! }
//! ````
//!
//! ## Macro Examples
//!
//! ```rust
//! use run_script::ScriptOptions;
//!
//! fn main() {
//!     // simple call to run script with only the script text
//!     let (code, output, error) = run_script::run_script!(
//!         r#"
//!          echo "Test"
//!          exit 0
//!          "#
//!     )
//!     .unwrap();
//!
//!     println!("Exit Code: {}", code);
//!     println!("Output: {}", output);
//!     println!("Error: {}", error);
//!
//!     // run script invoked with the script text and options
//!     let options = ScriptOptions::new();
//!     let (code, output, error) = run_script::run_script!(
//!         r#"
//!          echo "Test"
//!          exit 0
//!          "#,
//!         &options
//!     )
//!     .unwrap();
//!
//!     println!("Exit Code: {}", code);
//!     println!("Output: {}", output);
//!     println!("Error: {}", error);
//!
//!     // run script invoked with all arguments
//!     let options = ScriptOptions::new();
//!     let (code, output, error) = run_script::run_script!(
//!         r#"
//!          echo "Test"
//!          exit 0
//!          "#,
//!         &vec!["ARG1".to_string(), "ARG2".to_string()],
//!         &options
//!     )
//!     .unwrap();
//!
//!     println!("Exit Code: {}", code);
//!     println!("Output: {}", output);
//!     println!("Error: {}", error);
//!
//!     // spawn_script! works the same as run_script! but returns the child process handle
//!     let child = run_script::spawn_script!(
//!         r#"
//!          echo "Test"
//!          exit 0
//!          "#
//!     )
//!     .unwrap();
//!
//!     println!("PID: {}", child.id());
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! run_script = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/run_script/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/run_script/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

#[macro_use]
mod macros;
mod runner;
pub mod types;

use crate::types::ScriptResult;
use std::process::Child;

/// Error struct
pub type ScriptError = types::ScriptError;

/// Options available for invoking the script
pub type ScriptOptions = types::ScriptOptions;

/// Io Options available for invoking the script
pub type IoOptions = types::IoOptions;

/// Invokes the provided script content and returns the invocation output.
///
/// # Arguments
///
/// * `script` - The script content
/// * `args` - The script command line arguments
/// * `options` - Options provided to the script runner
///
/// # Example
///
/// ````
/// use run_script::ScriptOptions;
///
/// fn main() {
///     let options = ScriptOptions::new();
///
///     let args = vec![];
///
///     let (code, output, error) = run_script::run(
///         r#"
///         echo "Directory Info:"
///         dir
///         "#,
///         &args,
///         &options
///     ).unwrap();
///
///     println!("Exit Code: {}", code);
///     println!("Output: {}", output);
///     println!("Error: {}", error);
/// }
/// ````
pub fn run(
    script: &str,
    args: &Vec<String>,
    options: &ScriptOptions,
) -> ScriptResult<(i32, String, String)> {
    runner::run(script, &args, &options)
}

/// Invokes the provided script content and returns a process handle.
///
/// # Arguments
///
/// * `script` - The script content
/// * `args` - The script command line arguments
/// * `options` - Options provided to the script runner
///
/// # Example
///
/// ````
/// use run_script::ScriptOptions;
///
/// fn main() {
///     let options = ScriptOptions::new();
///
///     let args = vec![];
///
///     let child = run_script::spawn(
///         r#"
///         echo "Directory Info:"
///         dir
///         "#,
///         &args,
///         &options
///     ).unwrap();
/// }
/// ````
pub fn spawn(script: &str, args: &Vec<String>, options: &ScriptOptions) -> ScriptResult<Child> {
    runner::spawn(script, &args, &options)
}

/// Invokes the provided script content and returns the invocation output.
/// In case of invocation error or error exit code, this function will exit the main process.
///
/// # Arguments
///
/// * `script` - The script content
/// * `args` - The script command line arguments
/// * `options` - Options provided to the script runner
///
/// # Example
///
/// ````
/// use run_script::ScriptOptions;
///
/// fn main() {
///     let options = ScriptOptions::new();
///
///     let args = vec![];
///
///     let (output, error) = run_script::run_or_exit(
///         r#"
///         echo "Hello World"
///         "#,
///         &args,
///         &options
///     );
///
///     println!("Output: {}", output);
///     println!("Error: {}", error);
/// }
/// ````
pub fn run_or_exit(script: &str, args: &Vec<String>, options: &ScriptOptions) -> (String, String) {
    runner::run_or_exit(script, &args, &options)
}
