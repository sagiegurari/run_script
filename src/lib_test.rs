use super::*;
use doc_comment as _;

#[test]
fn run_test() {
    let args = vec![];
    let options = ScriptOptions::new();

    let (code, output, error) = run(
        r#"
        echo "Test"
        exit 0
        "#,
        &args,
        &options,
    )
    .unwrap();

    assert_eq!(code, 0);
    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn spawn_test_valid_exit_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    let child = spawn(
        r#"
        echo "Test"
        exit 0
        "#,
        &args,
        &options,
    )
    .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
}

#[test]
#[should_panic]
fn run_or_exit_error_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    run_or_exit("exit 1", &args, &options);
}

#[test]
fn run_or_exit_pipe_output() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.output_redirection = IoOptions::Pipe;

    let (output, error) = run_or_exit(
        r#"
        echo "Test"
        exit 0
        "#,
        &args,
        &options,
    );

    assert!(!output.is_empty());
    assert!(error.is_empty());
}
