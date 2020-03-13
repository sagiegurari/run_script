use run_script;
use run_script::ScriptOptions;

#[test]
fn run_macro_no_args_no_options_valid() {
    let (code, output, error) = run_script::run_script!(
        r#"
        echo "Test"
        exit 0
        "#
    )
    .unwrap();

    assert_eq!(code, 0);
    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn run_macro_no_args_no_options_error_output() {
    let output = run_script::run_script!(
        r#"
        echo "Test"
        exit 123
        "#
    )
    .unwrap();
    let code = output.0;

    assert_eq!(code, 123);
}

#[test]
fn run_macro_no_args_with_options() {
    let options = ScriptOptions::new();

    let (code, output, error) = run_script::run_script!(
        r#"
        echo "Test"
        exit 0
        "#,
        options
    )
    .unwrap();

    assert_eq!(code, 0);
    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn run_macro_with_args_with_options() {
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

    let (code, output, error) = run_script::run_script!(
        &script,
        &vec!["ARG1".to_string(), "ARG2".to_string()],
        options
    )
    .unwrap();

    assert_eq!(code, 0);
    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);

    assert!(output.find("arg1: ARG1").is_some());
    assert!(output.find("arg2: ARG2").is_some());
}
