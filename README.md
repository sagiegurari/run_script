# run_script

[![crates.io](https://img.shields.io/crates/v/run_script.svg)](https://crates.io/crates/run_script) [![Build Status](https://travis-ci.org/sagiegurari/run_script.svg)](http://travis-ci.org/sagiegurari/run_script) [![Build status](https://ci.appveyor.com/api/projects/status/yrb4y9cbaf6wtlk7?svg=true)](https://ci.appveyor.com/project/sagiegurari/run-script) [![codecov](https://codecov.io/gh/sagiegurari/run_script/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/run_script)<br>
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
Simply include the library and invoke the run function with the script text and run options:

````rust
extern crate run_script;

use run_script::ScriptOptions;

fn main() {
    let mut options = ScriptOptions::new();
    options.runner = None; // The script runner, for example bash. By default for windows it's cmd.exe and for other systems it is sh.
    options.capture_output = true; // True to capture and return the output. False will print it to the parent process output.
    options.exit_on_error = false; // Added set -e option (not available for windows)
    options.print_commands = false; // Added set -x option (not available for windows)

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
````

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
| 2017-11-04  | v0.1.2  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.