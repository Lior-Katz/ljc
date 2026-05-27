use ljc::error::SourceWithDiagnostic;
use ljc::lexer::{Token, Tokens};
use ljc::parser::Parser;
use std::fs;
use std::path::Path;

fn lex_to_string(path: &Path) -> datatest_stable::Result<String> {
    let input = fs::read_to_string(path)?;
    let mut tokens = Tokens::new(&input);
    let mut out = Vec::new();

    loop {
        match tokens.next().map(|(t, _)| t).map_err(|(e, _)| e) {
            Ok(Token::EOF) => break,
            Ok(token) => out.push(format!("{:?}", token)),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(out.join("\n"))
}

fn lexer_snapshot_test(path: &Path) -> datatest_stable::Result<()> {
    match lex_to_string(&path) {
        Ok(output) => {
            let name = path.file_stem().unwrap().to_str().unwrap();
            insta::with_settings!({
                snapshot_path => "lexer/snapshots",
                omit_expression => true,
            }, {
                insta::assert_snapshot!(name, output);
            });
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn parser_snapshot_test(path: &Path) -> datatest_stable::Result<()> {
    let input = fs::read_to_string(path)?;
    let program = Parser::new(&input)
        .parse()
        .map_err(|e| SourceWithDiagnostic::new(path, &input, e).to_string())?;
    let name = path.file_stem().unwrap().to_str().unwrap();
    insta::with_settings!({
        snapshot_path => "parser/snapshots",
        omit_expression => true,
    }, {
        insta::assert_snapshot!(name, program.to_string());
    });
    Ok(())
}

datatest_stable::harness! {
    { test = lexer_snapshot_test, root = "tests/snapshot/lexer", pattern = r"java" },
    { test = parser_snapshot_test, root = "tests/snapshot/parser", pattern = r"java" },
}
