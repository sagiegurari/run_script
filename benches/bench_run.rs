#![feature(test)]
extern crate run_script;
extern crate test;

use run_script::ScriptOptions;
use test::Bencher;

#[bench]
fn run(bencher: &mut Bencher) {
    let options = ScriptOptions::new();

    let args = vec![];

    bencher.iter(|| {
        let (code, output, error) = run_script::run(
            r#"
            echo "Test"
            exit 0
            "#,
            &args,
            &options,
        ).unwrap();

        assert_eq!(code, 0);
        assert!(output.len() > 0);
        assert_eq!(error.len(), 0);
    });
}
