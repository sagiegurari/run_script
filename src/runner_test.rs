use super::*;
use std::env::current_dir;
use std::path::Path;

#[test]
fn create_script_file_and_delete() {
    let file = create_script_file(&"test".to_string()).unwrap();
    assert!(Path::new(&file).exists());
    delete_file(&file);
    assert!(!Path::new(&file).exists());
}

#[test]
fn modify_script_no_shebang_default_options() {
    let options = ScriptOptions::new();

    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = modify_script(&"echo test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[test]
fn modify_script_with_shebang_default_options() {
    let options = ScriptOptions::new();

    let cwd = current_dir().unwrap();
    let mut expected_script = "#!/bin/bash\n".to_string();
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = modify_script(&"#!/bin/bash\necho test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[test]
fn modify_script_exit_on_error() {
    let mut options = ScriptOptions::new();
    options.exit_on_error = true;

    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    if !cfg!(windows) {
        expected_script.push_str("set -e\n");
    }
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = modify_script(&"echo test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[test]
fn modify_script_print_commands() {
    let mut options = ScriptOptions::new();
    options.print_commands = true;

    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    if !cfg!(windows) {
        expected_script.push_str("set -x\n");
    }
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = modify_script(&"echo test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[test]
fn modify_script_exit_on_error_and_print_commands() {
    let mut options = ScriptOptions::new();
    options.exit_on_error = true;
    options.print_commands = true;

    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    if !cfg!(windows) {
        expected_script.push_str("set -e\n");
        expected_script.push_str("set -x\n");
    }
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = modify_script(&"echo test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[test]
fn run_test_no_args_default_options() {
    let args = vec![];
    let options = ScriptOptions::new();

    let (code, output, error) = run(
        r#"
        echo "Test"
        exit 0
        "#,
        &args,
        &options
    ).unwrap();

    assert_eq!(code, 0);
    assert!(output.len() > 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn run_test_error_exit_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    let (code, output, error) = run("exit 1", &args, &options).unwrap();

    assert_eq!(code, 1);
    assert_eq!(output.len(), 0);
    assert_eq!(error.len(), 0);
}

#[test]
fn run_test_error_execute() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.runner = Some("badtest123".to_string());

    let result = run("exit 1", &args, &options);
    assert!(result.is_err());
}
