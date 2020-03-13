//! # macros
//!
//! Defines the library macros
//!

/// Enables to invoke the run_script::run function more easily without providing all input.
///
/// # Arguments
///
/// * `script` - The script content
/// * `args` - Optional, script command line arguments. If provided, the last options argument must also be provided.
/// * `options` - Optional, options provided to the script runner
///
/// # Examples
///
/// ```rust
/// use run_script::ScriptOptions;
///
/// fn main() {
///     // simple call to run script with only the script text
///     let (code, output, error) = run_script::run_script!(
///         r#"
///         echo "Test"
///         exit 0
///         "#
///     ).unwrap();
///
///     // run script invoked with the script text and options
///     let options = ScriptOptions::new();
///     let (code, output, error) = run_script::run_script!(
///         r#"
///         echo "Test"
///         exit 0
///         "#,
///         &options
///     ).unwrap();
///
///     // run script invoked with all arguments
///     let options = ScriptOptions::new();
///     let (code, output, error) = run_script::run_script!(
///         r#"
///         echo "Test"
///         exit 0
///         "#,
///         &vec!["ARG1".to_string(), "ARG2".to_string()],
///         &options
///     ).unwrap();
/// }
/// ```
#[macro_export]
macro_rules! run_script {
    ($script:expr) => {{
        let args = vec![];
        let options = $crate::ScriptOptions::new();
        $crate::run(&$script, &args, &options)
    }};
    ($script:expr, $options:expr) => {{
        let args = vec![];
        $crate::run(&$script, &args, &$options)
    }};
    ($script:expr, $args:expr, $options:expr) => {{
        $crate::run(&$script, &$args, &$options)
    }};
}

/// Enables to invoke the run_script::spawn function more easily without providing all input.
///
/// # Arguments
///
/// * `script` - The script content
/// * `args` - Optional, script command line arguments. If provided, the last options argument must also be provided.
/// * `options` - Optional, options provided to the script runner
///
/// # Examples
///
/// ```rust_script;
///
/// use run_script::ScriptOptions;
///
/// fn main() {
///     // simple call to run script with only the script text
///     let child = run_script::spawn_script!(
///         r#"
///         echo "Test"
///         exit 0
///         "#
///     ).unwrap();
///
///     // run script invoked with the script text and options
///     let options = ScriptOptions::new();
///     let child = run_script::spawn_script!(
///         r#"
///         echo "Test"
///         exit 0
///         "#,
///         &options
///     ).unwrap();
///
///     // run script invoked with all arguments
///     let options = ScriptOptions::new();
///     let child = run_script::spawn_script!(
///         r#"
///         echo "Test"
///         exit 0
///         "#,
///         &vec!["ARG1".to_string(), "ARG2".to_string()],
///         &options
///     ).unwrap();
/// }
/// ```
#[macro_export]
macro_rules! spawn_script {
    ($script:expr) => {{
        let args = vec![];
        let options = $crate::ScriptOptions::new();
        $crate::spawn(&$script, &args, &options)
    }};
    ($script:expr, $options:expr) => {{
        let args = vec![];
        $crate::spawn(&$script, &args, &$options)
    }};
    ($script:expr, $args:expr, $options:expr) => {{
        $crate::spawn(&$script, &$args, &$options)
    }};
}

/// Enables to invoke the run_script::run_or_exit function more easily without providing all input.
///
/// # Arguments
///
/// * `script` - The script content
/// * `args` - Optional, script command line arguments. If provided, the last options argument must also be provided.
/// * `options` - Optional, options provided to the script runner
///
/// # Examples
///
/// ```rust
/// use run_script::ScriptOptions;
///
/// fn main() {
///     // simple call to the macro with only the script text
///     let (output, error) = run_script::run_script_or_exit!(
///         r#"
///         echo "Test"
///         exit 0
///         "#
///     );
///
///     // macro invoked with the script text and options
///     let options = ScriptOptions::new();
///     let (output, error) = run_script::run_script_or_exit!(
///         r#"
///         echo "Test"
///         exit 0
///         "#,
///         &options
///     );
///
///     // macro invoked with all arguments
///     let options = ScriptOptions::new();
///     let (output, error) = run_script::run_script_or_exit!(
///         r#"
///         echo "Test"
///         exit 0
///         "#,
///         &vec!["ARG1".to_string(), "ARG2".to_string()],
///         &options
///     );
/// }
/// ```
#[macro_export]
macro_rules! run_script_or_exit {
    ($script:expr) => {{
        let args = vec![];
        let options = $crate::ScriptOptions::new();
        $crate::run_or_exit(&$script, &args, &options)
    }};
    ($script:expr, $options:expr) => {{
        let args = vec![];
        $crate::run_or_exit(&$script, &args, &$options)
    }};
    ($script:expr, $args:expr, $options:expr) => {{
        $crate::run_or_exit(&$script, &$args, &$options)
    }};
}
