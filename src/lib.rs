#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    array_into_iter,
    bindings_with_variant_name,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incomplete_features,
    indirect_structural_match,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_patterns,
    path_statements,
    patterns_in_fns_without_body,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolon,
    safe_packed_borrows,
    soft_unstable,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_recursion,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    intra_doc_link_resolution_failure,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

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
) -> Result<(i32, String, String), ScriptError> {
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
pub fn spawn(
    script: &str,
    args: &Vec<String>,
    options: &ScriptOptions,
) -> Result<Child, ScriptError> {
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
