#[macro_use]
extern crate run_script;

use run_script::ScriptOptions;

fn main() {
    // simple call to run script with only the script text
    let (code, output, error) = run_script!(
        r#"
         echo "Test"
         exit 0
         "#
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    // run script invoked with the script text and options
    let options = ScriptOptions::new();
    let (code, output, error) = run_script!(
        r#"
         echo "Test"
         exit 0
         "#,
        &options
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    // run script invoked with all arguments
    let options = ScriptOptions::new();
    let (code, output, error) = run_script!(
        r#"
         echo "Test"
         exit 0
         "#,
        &vec!["ARG1".to_string(), "ARG2".to_string()],
        &options
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    // spawn_script! works the same as run_script! but returns the child process handle
    let child = spawn_script!(
        r#"
         echo "Test"
         exit 0
         "#
    )
    .unwrap();

    println!("PID: {}", child.id());
}
