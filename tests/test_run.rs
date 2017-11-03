extern crate run_script;

use run_script::ScriptOptions;

#[test]
fn run() {
    let options = ScriptOptions::new();

    let (code, output, error) = run_script::run(
        r#"
        echo "Directory Info:"
        dir
        "#,
        &vec![],
        &options
    ).unwrap();

    assert_eq!(code, 0);
    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}
