#[macro_use]
extern crate run_script;

use run_script::ScriptOptions;

#[test]
fn spawn_macro_no_args_no_options_valid() {
    let child = spawn_script!(
        r#"
        echo "Test"
        exit 0
        "#
    )
    .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
}

#[test]
fn spawn_macro_no_args_no_options_error_output() {
    let child = spawn_script!(
        r#"
        echo "Test"
        exit 123
        "#
    )
    .unwrap();

    let output = child.wait_with_output().unwrap();

    assert_eq!(output.status.code().unwrap(), 123);
}

#[test]
fn spawn_macro_no_args_with_options() {
    let options = ScriptOptions::new();

    let child = spawn_script!(
        r#"
        echo "Test"
        exit 0
        "#,
        options
    )
    .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert!(output.stdout.len() > 0);
}

#[test]
fn spawn_macro_with_args_with_options() {
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

    let child = spawn_script!(
        &script,
        &vec!["ARG1".to_string(), "ARG2".to_string()],
        options
    )
    .unwrap();

    let output = child.wait_with_output().unwrap();

    assert!(output.status.success());
    assert!(output.stdout.len() > 0);
    assert_eq!(output.stderr.len(), 0);
}
