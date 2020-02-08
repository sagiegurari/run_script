#[macro_use]
extern crate run_script;

use run_script::ScriptOptions;

#[test]
fn run_script_or_exit_macro_no_args_no_options_valid() {
    let (output, error) = run_script_or_exit!(
        r#"
        echo "Test"
        exit 0
        "#
    );

    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn run_script_or_exit_macro_no_args_with_options() {
    let options = ScriptOptions::new();

    let (output, error) = run_script_or_exit!(
        r#"
        echo "Test"
        exit 0
        "#,
        options
    );

    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn run_script_or_exit_macro_with_args_with_options() {
    let options = ScriptOptions::new();

    let script = if cfg!(windows) {
        r#"
        echo arg1: %1
        echo arg2: %2
        exit 0
        "#
    } else {
        r#"
        echo arg1: $1
        echo arg2: $2
        exit 0
        "#
    };

    let (output, error) = run_script_or_exit!(
        &script,
        &vec!["ARG1".to_string(), "ARG2".to_string()],
        options
    );

    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);

    assert!(output.find("arg1: ARG1").is_some());
    assert!(output.find("arg2: ARG2").is_some());
}
