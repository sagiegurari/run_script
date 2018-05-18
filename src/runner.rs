//! # command
//!
//! Runs task commands/scripts.
//!

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use rand::{thread_rng, Rng};
use std::env;
use std::env::current_dir;
use std::fs::{create_dir_all, remove_file, File};
use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::process::{Command, ExitStatus, Output, Stdio};
use types::{ErrorInfo, ScriptError, ScriptOptions};

#[cfg(not(windows))]
use users::get_current_username;

/// Returns the exit code
fn get_exit_code(code: ExitStatus) -> i32 {
    if !code.success() {
        match code.code() {
            Some(value) => value,
            None => -1,
        }
    } else {
        0
    }
}

/// Runs the requested command and return its output.
fn run_command(
    command_string: &str,
    args: &Vec<String>,
    options: &ScriptOptions,
) -> io::Result<Output> {
    let mut command = Command::new(&command_string);

    for arg in args.iter() {
        command.arg(arg);
    }

    command.stdin(Stdio::inherit());
    if !options.capture_output {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }

    command.output()
}

fn delete_file(file: &str) {
    remove_file(file).unwrap_or(());
}

#[cfg(windows)]
fn get_additional_temp_path() -> Option<String> {
    None
}

#[cfg(not(windows))]
fn get_additional_temp_path() -> Option<String> {
    get_current_username()
}

fn create_script_file(script: &String) -> Result<String, Error> {
    let name = env!("CARGO_PKG_NAME");
    let file_name: String = thread_rng().gen_ascii_chars().take(10).collect();

    let mut file_path = env::temp_dir();

    match get_additional_temp_path() {
        Some(additional_path) => file_path.push(additional_path),
        None => {}
    };

    file_path.push(name);

    // create parent directory
    match create_dir_all(&file_path) {
        Ok(_) => {
            file_path.push(file_name);
            if cfg!(windows) {
                file_path.set_extension("bat");
            } else {
                file_path.set_extension("sh");
            };

            let file_path_str = &file_path.to_str().unwrap_or("");

            match File::create(&file_path) {
                Ok(mut file) => match file.write_all(script.as_bytes()) {
                    Ok(_) => Ok(file_path_str.to_string()),
                    Err(error) => {
                        delete_file(&file_path_str);

                        Err(error)
                    }
                },
                Err(error) => Err(error),
            }
        }
        Err(error) => Err(error),
    }
}

fn modify_script(script: &String, options: &ScriptOptions) -> Result<String, ScriptError> {
    match current_dir() {
        Ok(cwd_holder) => {
            match cwd_holder.to_str() {
                Some(cwd) => {
                    // create cd command
                    let mut cd_command = "cd ".to_string();
                    cd_command.push_str(cwd);

                    let mut script_lines: Vec<String> = script
                        .trim()
                        .split("\n")
                        .map(|string| string.to_string())
                        .collect();

                    // check if first line is shebang line
                    let mut insert_index =
                        if script_lines.len() > 0 && script_lines[0].starts_with("#!") {
                            1
                        } else {
                            0
                        };

                    if !cfg!(windows) {
                        if options.exit_on_error {
                            script_lines.insert(insert_index, "set -e".to_string());
                            insert_index = insert_index + 1;
                        }

                        if options.print_commands {
                            script_lines.insert(insert_index, "set -x".to_string());
                            insert_index = insert_index + 1;
                        }
                    }

                    script_lines.insert(insert_index, cd_command);

                    script_lines.push("\n".to_string());

                    let updated_script = script_lines.join("\n");

                    Ok(updated_script)
                }
                None => Err(ScriptError {
                    info: ErrorInfo::Description(
                        "Unable to extract current working directory path.",
                    ),
                }),
            }
        }
        Err(error) => Err(ScriptError {
            info: ErrorInfo::IOError(error),
        }),
    }
}

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
/// extern crate run_script;
///
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
pub(crate) fn run(
    script: &str,
    args: &Vec<String>,
    options: &ScriptOptions,
) -> Result<(i32, String, String), ScriptError> {
    match modify_script(&script.to_string(), &options) {
        Ok(updated_script) => match create_script_file(&updated_script) {
            Ok(file) => {
                let command = match options.runner {
                    Some(ref value) => value,
                    None => {
                        if cfg!(windows) {
                            "cmd.exe"
                        } else {
                            "sh"
                        }
                    }
                };

                let mut all_args = if cfg!(windows) {
                    vec!["/C".to_string(), file.to_string()]
                } else {
                    vec![file.to_string()]
                };

                all_args.extend(args.iter().cloned());

                let result = run_command(&command, &all_args, &options);

                delete_file(&file);

                match result {
                    Ok(output) => {
                        let exit_code = get_exit_code(output.status);
                        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

                        Ok((exit_code, stdout, stderr))
                    }
                    Err(error) => Err(ScriptError {
                        info: ErrorInfo::IOError(error),
                    }),
                }
            }
            Err(error) => Err(ScriptError {
                info: ErrorInfo::IOError(error),
            }),
        },
        Err(error) => Err(error),
    }
}
