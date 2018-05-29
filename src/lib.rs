#![deny(
    anonymous_parameters, const_err, dead_code, deprecated, exceeding_bitshifts,
    illegal_floating_point_literal_pattern, improper_ctypes, incoherent_fundamental_impls,
    invalid_type_param_default, late_bound_lifetime_arguments, legacy_constructor_visibility,
    legacy_directory_ownership, missing_copy_implementations, missing_docs,
    missing_fragment_specifier, mutable_transmutes, no_mangle_const_items, no_mangle_generic_items,
    non_camel_case_types, non_shorthand_field_patterns, non_snake_case, non_upper_case_globals,
    overflowing_literals, parenthesized_params_in_types_and_modules, path_statements,
    patterns_in_fns_without_body, plugin_as_library, private_in_public, private_no_mangle_fns,
    private_no_mangle_statics, pub_use_of_private_extern_crate, safe_extern_statics,
    safe_packed_borrows, stable_features, trivial_casts, trivial_numeric_casts, type_alias_bounds,
    tyvar_behind_raw_pointer, unconditional_recursion, unions_with_drop_fields, unknown_crate_types,
    unreachable_code, unreachable_patterns, unreachable_pub, unsafe_code, unstable_features,
    unstable_name_collisions, unused_allocation, unused_assignments, unused_attributes,
    unused_comparisons, unused_doc_comments, unused_extern_crates, unused_features,
    unused_import_braces, unused_imports, unused_macros, unused_must_use, unused_mut, unused_parens,
    unused_qualifications, unused_unsafe, unused_variables, while_true
)]
#![warn(unknown_lints)]
#![allow(
    bare_trait_objects, box_pointers, elided_lifetimes_in_paths, missing_debug_implementations,
    single_use_lifetimes, unused_results, variant_size_differences, warnings,
    renamed_and_removed_lints
)]
#![cfg_attr(feature = "clippy", feature(plugin))]

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
//! ````
//! extern crate run_script;
//!
//! use run_script::ScriptOptions;
//!
//! fn main() {
//!     let options = ScriptOptions::new();
//!
//!     let args = vec![];
//!
//!     let (code, output, error) = run_script::run(
//!         r#"
//!         echo "Directory Info:"
//!         dir
//!         "#,
//!         &args,
//!         &options
//!     ).unwrap();
//!
//!     println!("Exit Code: {}", code);
//!     println!("Output: {}", output);
//!     println!("Error: {}", error);
//! }
//! ````
//!
//! ## Macro Examples
//!
//! ```rust
//! #[macro_use]
//! extern crate run_script;
//!
//! use run_script::ScriptOptions;
//!
//! fn main() {
//!     // simple call to run script with only the script text
//!     let (code, output, error) = run_script!(
//!         r#"
//!         echo "Test"
//!         exit 0
//!         "#
//!     ).unwrap();
//!
//!     // run script invoked with the script text and options
//!     let options = ScriptOptions::new();
//!     let (code, output, error) = run_script!(
//!         r#"
//!         echo "Test"
//!         exit 0
//!         "#,
//!         &options
//!     ).unwrap();
//!
//!     // run script invoked with all arguments
//!     let options = ScriptOptions::new();
//!     let (code, output, error) = run_script!(
//!         r#"
//!         echo "Test"
//!         exit 0
//!         "#,
//!         &vec!["ARG1".to_string(), "ARG2".to_string()],
//!         &options
//!     ).unwrap();
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

extern crate rand;

#[cfg(not(windows))]
extern crate users;

#[macro_use]
mod macros;
mod runner;
pub mod types;

/// Error struct
pub type ScriptError = types::ScriptError;

/// Options available for invoking the script
pub type ScriptOptions = types::ScriptOptions;

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
pub fn run(
    script: &str,
    args: &Vec<String>,
    options: &ScriptOptions,
) -> Result<(i32, String, String), ScriptError> {
    runner::run(script, &args, &options)
}
