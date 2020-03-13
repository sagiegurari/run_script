use run_script;
use run_script::ScriptOptions;

fn main() {
    let options = ScriptOptions::new();

    let args = vec![];

    // run the script and get the script execution output
    let (code, output, error) = run_script::run(
        r#"
         echo "Directory Info:"
         dir
         "#,
        &args,
        &options,
    )
    .unwrap();

    println!("Exit Code: {}", code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    // run the script and get a handle to the running child process
    let child = run_script::spawn(
        r#"
         echo "Directory Info:"
         dir
         "#,
        &args,
        &options,
    )
    .unwrap();

    let spawn_output = child.wait_with_output().unwrap();

    println!("Success: {}", &spawn_output.status.success());
}
