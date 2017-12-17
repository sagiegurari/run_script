use super::*;
use std::error::Error;
use std::io::Write;

#[test]
fn script_error_description() {
    let script_error = ScriptError {
        info: ErrorInfo::Description("test"),
    };

    assert_eq!(script_error.description(), "test");
    assert!(script_error.cause().is_none());

    let mut writer = Vec::new();
    write!(&mut writer, "formatted {}", script_error).unwrap();
    assert_eq!(writer, b"formatted test");
}

#[test]
fn script_options_new() {
    let options = ScriptOptions::new();

    assert!(options.runner.is_none());
    assert!(options.capture_output);
    assert!(!options.exit_on_error);
    assert!(!options.print_commands);
}
