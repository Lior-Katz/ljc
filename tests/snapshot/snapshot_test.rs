use std::path::Path;

fn lexer_snapshot_test(_path: &Path) -> datatest_stable::Result<()> {
    // ... write test here
    Ok(())
}

datatest_stable::harness! {
    { test = lexer_snapshot_test, root = "tests/snapshot/lexer", pattern = r".*\.java" },
}