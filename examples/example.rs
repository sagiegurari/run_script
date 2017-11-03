extern crate run_script;

use run_script::ScriptOptions;

fn main() {
    let options = ScriptOptions::new();

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
