use super::*;
use std::error::Error;
use std::io::Write;

#[test]
fn script_error_description() {
    let script_error = ScriptError::Description("test");

    assert_eq!(script_error.to_string(), "test");
    assert!(script_error.source().is_none());

    let mut writer = Vec::new();
    write!(&mut writer, "formatted {}", script_error).unwrap();
    assert_eq!(writer, b"formatted test");
}

#[test]
fn script_options_new() {
    let options = ScriptOptions::new();

    assert!(options.runner.is_none());
    assert!(options.working_directory.is_none());
    assert_eq!(options.input_redirection, IoOptions::Inherit);
    assert_eq!(options.output_redirection, IoOptions::Pipe);
    assert!(!options.exit_on_error);
    assert!(!options.print_commands);
    #[cfg(feature = "encoding_rs")]
    assert!(options.encoding.is_none());
}
