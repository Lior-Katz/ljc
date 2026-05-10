use ljc::parser::Parser;
use std::fs;
use std::path::{Path, PathBuf};

fn error_tests(path: &Path) -> datatest_stable::Result<()> {
    let input = fs::read_to_string(path)?;
    if let Err(e) = Parser::new(&input).parse() {
        let test_dir = path.parent().unwrap().file_name().unwrap();
        insta::with_settings!({
            omit_expression => true,
            snapshot_path => PathBuf::from("errors").join(test_dir),
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(path.file_stem().unwrap().to_str().unwrap(), e.to_string());
        });
        Ok(())
    } else {
        Err("Compilation succeeded".into())
    }
}

datatest_stable::harness! {
    { test = error_tests, root = "tests/ui/errors", pattern = r"\.java$" },
}
