# run_script

[![crates.io](https://img.shields.io/crates/v/run_script.svg)](https://crates.io/crates/run_script) [![CI](https://github.com/sagiegurari/run_script/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/run_script/actions) [![codecov](https://codecov.io/gh/sagiegurari/run_script/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/run_script)<br>
[![license](https://img.shields.io/crates/l/run_script.svg)](https://github.com/sagiegurari/run_script/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/run_script.svg)](https://libraries.io/cargo/run_script) [![Documentation](https://docs.rs/run_script/badge.svg)](https://docs.rs/crate/run_script/) [![downloads](https://img.shields.io/crates/d/run_script.svg)](https://crates.io/crates/run_script)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Run shell scripts in [rust](https://www.rust-lang.org/).

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/run_script/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
This library enables to invoke shell scripts based on their content.<br>
While std::process::Command works great to execute standalone command, you need more manual code to take a script text and execute it.<br>
For this purpose, this library was created.

<a name="usage"></a>
## Usage
Simply include the library and invoke the run/spawn function with the script text and run options:

<!--{ "examples/function_examples.rs" | lines: 2 | code: rust }-->
```rust
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
```
<!--{ end }-->

The library also provides the **run_script!**,  **spawn_script!** and **run_script_or_exit!** macros for simpler usage.

<!--{ "examples/macro_examples.rs" | lines: 2 | code: rust }-->
```rust
use run_script::ScriptOptions;

fn main() {
    // simple call to run script with only the script text
    let (code, output, error) = run_script::run_script!(
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
    let (code, output, error) = run_script::run_script!(
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
    let (code, output, error) = run_script::run_script!(
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
    let child = run_script::spawn_script!(
        r#"
         echo "Test"
         exit 0
         "#
    )
    .unwrap();

    println!("PID: {}", child.id());
}
```
<!--{ end }-->

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
run_script = "^0.10.0"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/run_script/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
