use super::*;

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
