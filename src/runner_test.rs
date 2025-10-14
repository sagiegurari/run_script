use super::*;
use std::env::current_dir;
use std::path::{Path, PathBuf};

#[test]
fn create_script_file_and_delete() {
    let file = create_script_file(&"test".to_string()).unwrap();
    assert!(Path::new(&file).exists());
    fsio::file::delete_ignore_error(&file);
    assert!(!Path::new(&file).exists());
}

#[test]
fn modify_script_no_shebang_default_options() {
    let options = ScriptOptions::new();

    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    if cfg!(windows) {
        expected_script.push_str("@echo off\n");
    }
    expected_script.push_str("cd \"");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\"\necho test\n\n");

    let script = modify_script(&"echo test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[cfg(not(windows))]
#[test]
fn modify_script_with_shebang_default_options() {
    let options = ScriptOptions::new();

    let cwd = current_dir().unwrap();
    let mut expected_script = "#!/bin/bash\n".to_string();
    expected_script.push_str("cd \"");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\"\necho test\n\n");

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
    } else {
        expected_script.push_str("@echo off\n");
    }
    expected_script.push_str("cd \"");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\"\necho test\n\n");

    let script = modify_script(&"echo test".to_string(), &options).unwrap();

    assert_eq!(script, expected_script);
}

#[test]
fn modify_script_working_directory() {
    let mut options = ScriptOptions::new();
    options.working_directory = Some(PathBuf::from("/usr/me/home"));

    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    if cfg!(windows) {
        expected_script.push_str("@echo off\n");
    }
    expected_script.push_str("cd \"");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\" && cd \"/usr/me/home\"\necho test\n\n");

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
    expected_script.push_str("cd \"");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\"\necho test\n\n");

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
    expected_script.push_str("cd \"");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\"\necho test\n\n");

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
        &options,
    )
    .unwrap();

    assert_eq!(code, 0);
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[cfg(feature = "encoding_rs")]
#[test]
fn run_test_no_args_with_encoding() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.encoding = Some(encoding_rs::UTF_8);

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
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_error_exit_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    let result = run("exit 1", &args, &options).unwrap();

    assert_eq!(result.0, 1);
}

#[test]
fn run_test_error_execute() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.runner = Some("badtest123".to_string());

    let result = run("exit 1", &args, &options);
    assert!(result.is_err());
}

#[test]
fn run_test_with_runner_args() {
    let args = vec![];
    let mut options = ScriptOptions::new();

    if cfg!(windows) {
        options.runner = Some("powershell".to_string());
        options.runner_args = Some(vec![
            "-window".to_string(),
            "normal".to_string(),
            "-command".to_string(),
        ]);
    } else {
        options.runner_args = Some(vec!["--".to_string()]);
    }

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
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_with_bad_runner_args() {
    let args = vec![];
    let mut options = ScriptOptions::new();

    if cfg!(windows) {
        options.runner = Some("powershell".to_string());
        options.runner_args = Some(vec!["-notarealflag".to_string()]);
    } else {
        options.runner_args = Some(vec!["-notarealflag".to_string(), "--".to_string()]);
    }

    let (code, output, error) = run(
        r#"
        echo "Test"
        exit 0
        "#,
        &args,
        &options,
    )
    .unwrap();

    assert_ne!(code, 0);
    assert!(output.is_empty());
    assert!(!error.is_empty());
}

#[test]
fn run_test_with_args() {
    let args = vec!["ARG1".to_string(), "ARG2".to_string()];
    let options = ScriptOptions::new();

    let script = if cfg!(windows) {
        "echo arg1: %1\necho arg2: %2"
    } else {
        "echo arg1: $1\necho arg2: $2"
    };

    let (code, output, error) = run(script, &args, &options).unwrap();

    assert_eq!(code, 0);
    assert!(!output.is_empty());
    assert!(error.is_empty());

    assert!(output.find("arg1: ARG1").is_some());
    assert!(output.find("arg2: ARG2").is_some());
}

#[test]
fn run_test_with_runner_and_script_args() {
    let args = vec!["ARG1".to_string(), "ARG2".to_string()];
    let mut options = ScriptOptions::new();

    if cfg!(windows) {
        options.runner = Some("powershell".to_string());
        options.runner_args = Some(vec![
            "-window".to_string(),
            "normal".to_string(),
            "-command".to_string(),
        ]);
    } else {
        options.runner_args = Some(vec!["--".to_string()]);
    }

    let script = if cfg!(windows) {
        "echo arg1: %1\necho arg2: %2"
    } else {
        "echo arg1: $1\necho arg2: $2"
    };

    let (code, output, error) = run(script, &args, &options).unwrap();

    assert_eq!(code, 0);
    assert!(!output.is_empty());
    assert!(error.is_empty());

    assert!(output.find("arg1: ARG1").is_some());
    assert!(output.find("arg2: ARG2").is_some());
}

#[test]
fn run_test_no_args_inherit_input() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.input_redirection = IoOptions::Inherit;

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
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_no_args_pipe_input() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.input_redirection = IoOptions::Pipe;

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
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_no_args_null_input() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.input_redirection = IoOptions::Null;

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
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_no_args_inherit_output() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.output_redirection = IoOptions::Inherit;

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
    assert!(output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_no_args_pipe_output() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.output_redirection = IoOptions::Pipe;

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
    assert!(!output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn run_test_no_args_null_output() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    options.output_redirection = IoOptions::Null;

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
    assert!(output.is_empty());
    assert!(error.is_empty());
}

#[test]
fn spawn_test_valid_exit_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    let child = spawn("exit 0", &args, &options).unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
}

#[test]
fn spawn_test_error_exit_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    let child = spawn("exit 1", &args, &options).unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(!output.status.success());
}

#[test]
#[should_panic]
fn run_or_exit_error_code() {
    let args = vec![];
    let options = ScriptOptions::new();

    run_or_exit("exit 1", &args, &options);
}

#[test]
#[should_panic]
fn run_or_exit_invocation_error() {
    let args = vec![];
    let options = ScriptOptions::new();

    run_or_exit("badcommand", &args, &options);
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

#[test]
fn run_or_exit_append_env() {
    let args = vec![];
    let mut options = ScriptOptions::new();
    let mut env_vars = std::collections::HashMap::<String, String>::new();
    env_vars.insert("MY_TEST_VARIABLE".to_string(), "MY_TEST_VALUE".to_string());
    options.env_vars = Some(env_vars);

    std::env::set_var("PARENT_VAR", "PARENT_VALUE");

    let script: String;

    if cfg!(windows) {
        script = r#"
            ECHO %MY_TEST_VARIABLE%
            ECHO %PARENT_VAR%
        "#
        .to_string();
    } else {
        script = r#"
            echo $MY_TEST_VARIABLE
            echo $PARENT_VAR
        "#
        .to_string()
    }

    let (output, error) = run_or_exit(&script, &args, &options);

    assert!(output.contains("MY_TEST_VALUE"));
    assert!(output.contains("PARENT_VALUE"));
    assert!(error.is_empty());

    // Check if current environment is polluted
    match std::env::var("MY_TEST_VARIABLE") {
        Ok(_) => assert!(false, "The parent environment is polluted"),
        Err(_) => (),
    }
}
