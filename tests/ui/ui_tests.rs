use ljc::Error;
use ljc::error::{Diagnostic, SourceWithDiagnostic};
use ljc::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

fn run(input: &String) -> Result<(), Vec<Diagnostic<Error>>> {
    let _ = Parser::new(input).parse().map_err(|e| vec![e.into()])?;
    Ok(())
}

fn error_tests(path: &Path) -> datatest_stable::Result<()> {
    let input = fs::read_to_string(path)?;
    match run(&input) {
        Err(errors) => {
            let output = errors
                .into_iter()
                .map(|e| SourceWithDiagnostic::new(path, &input, e))
                .fold(String::new(), |acc, e| format!("{acc}{e}\n"));
            let test_dir = path.parent().unwrap().canonicalize()?;
            insta::with_settings!({
                omit_expression => true,
                snapshot_path => PathBuf::from("errors").join(test_dir),
                prepend_module_to_snapshot => false,
            }, {
                insta::assert_snapshot!(path.file_stem().unwrap().to_str().unwrap(), output);
            });
            Ok(())
        }
        Ok(_) => Err("Compilation succeeded".into()),
    }
}

datatest_stable::harness! {
    { test = error_tests, root = "tests/ui/errors", pattern = r"\.java$" },
}
