# run_script

[![crates.io](https://img.shields.io/crates/v/run_script.svg)](https://crates.io/crates/run_script) [![Build Status](https://travis-ci.org/sagiegurari/run_script.svg?branch=master)](http://travis-ci.org/sagiegurari/run_script) [![Build status](https://ci.appveyor.com/api/projects/status/github/sagiegurari/run_script?branch=master&svg=true)](https://ci.appveyor.com/project/sagiegurari/run-script) [![codecov](https://codecov.io/gh/sagiegurari/run_script/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/run_script)<br>
[![license](https://img.shields.io/crates/l/run_script.svg)](https://github.com/sagiegurari/run_script/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/run_script.svg)](https://libraries.io/cargo/run_script) [![Documentation](https://docs.rs/run_script/badge.svg)](https://docs.rs/crate/run_script/) [![downloads](https://img.shields.io/crates/d/run_script.svg)](https://crates.io/crates/run_script)<br>
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> Run shell scripts in [rust](https://www.rust-lang.org/).

* [Overview](#overview)
* [Usage](#usage)
* [Installation](#installation)
* [API Documentation](https://sagiegurari.github.io/run_script/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
This library enables to invoke shell scripts based on their content.<br>
While std::process::Command works great to execute standalone command, you need more manual code to take a script text and execute it.<br>
For this purpose, this library was created.

<a name="usage"></a>
## Usage
Simply include the library and invoke the run/spawn function with the script text and run options:

```rust
extern crate run_script;

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
        &options
    ).unwrap();

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
        &options
    ).unwrap();

    let spawn_output = child.wait_with_output().unwrap();

    println!("Success: {}", &spawn_output.status.success());
}
```

The library also provides the ```run_script!``` and ```spawn_script!``` macros for simpler usage.

```rust
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
    ).unwrap();

    // run script invoked with the script text and options
    let options = ScriptOptions::new();
    let (code, output, error) = run_script!(
        r#"
        echo "Test"
        exit 0
        "#,
        &options
    ).unwrap();

    // run script invoked with all arguments
    let options = ScriptOptions::new();
    let (code, output, error) = run_script!(
        r#"
        echo "Test"
        exit 0
        "#,
        &vec!["ARG1".to_string(), "ARG2".to_string()],
        &options
    ).unwrap();

    // spawn_script! works the same as run_script! but returns the child process handle
    let child = spawn_script!(
        r#"
        echo "Test"
        exit 0
        "#
    ).unwrap();
}
```

<a name="installation"></a>
## Installation
In order to use this library, just add it as a dependency:

```ini
[dependencies]
run_script = "*"
```

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/run_script/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2019-12-24  | v0.4.0  | New spawn function and spawn_script! macro #7 |
| 2018-03-20  | v0.1.14 | Fix permissions issue (#2) |
| 2017-12-23  | v0.1.10 | New run_script! macro |
| 2017-11-04  | v0.1.1  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
