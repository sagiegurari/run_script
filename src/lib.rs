#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bindings_with_variant_name,
    break_with_label_and_loop,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_err,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_cfg_attr_crate_type_name,
    deprecated_in_future,
    deref_into_dyn_supertrait,
    deref_nullptr,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    enum_intrinsics_non_enums,
    explicit_outlives_requirements,
    exported_private_dependencies,
    forbidden_lint_groups,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    invalid_atomic_ordering,
    invalid_doc_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_borrow_reservation_conflict,
    mutable_transmutes,
    named_asm_labels,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    stable_features,
    temporary_cstring_as_ptr,
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unaligned_references,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unsupported_calling_conventions,
    unsupported_naked_functions,
    unused_allocation,
    unused_assignments,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
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
    useless_deprecated,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
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
