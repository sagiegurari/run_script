extern crate run_script;

use run_script::ScriptOptions;

fn main() {
    let mut options = ScriptOptions::new();
    options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
    options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
    options.exit_on_error = false; // Adds set -e option (not available for windows)
    options.print_commands = false; // Adds set -x option (not available for windows)

    let args = vec![];

    let (code, output, error) = run_script::run(
        r#"
        echo "Directory Info:"
        dir
        "#,
        &args,
        &options
    ).unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);
}
