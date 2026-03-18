use ljc::lexer::lexer::lex_single_file;
use std::path::Path;

fn lex_to_string(path: &Path) -> Result<String, String> {
    let mut tokens = lex_single_file(path).map_err(|err| format!("{:?}", err))?;

    let mut out = Vec::new();

    loop {
        match tokens.next() {
            Ok(Some(tok)) => out.push(format!("{:?}", tok)),
            Ok(None) => break,
            Err(e) => return Err(format!("{:?}", e)),
        }
    }

    Ok(out.join("\n"))
}

fn lexer_snapshot_test(path: &Path) -> datatest_stable::Result<()> {
    let output = lex_to_string(&path).unwrap_or_else(|e| format!("LEX_ERROR:\n{}", e));

    // need to remove file extension because snapshots are saved in the dataset-stable root directory.
    // this means that otherwise dataset-stable (which runs on all java files) would pick up the snapshots as well.
    let name = path.file_stem().unwrap().to_str().unwrap();
    insta::with_settings!({
        snapshot_path => "lexer/snapshots",
        omit_expression => true,
    }, {
        insta::assert_snapshot!(name, output);
    });
    Ok(()) // to satisfy return type check. insta fails when the output doesn't match
}

datatest_stable::harness! {
    { test = lexer_snapshot_test, root = "tests/snapshot/lexer", pattern = r"java" },
}
