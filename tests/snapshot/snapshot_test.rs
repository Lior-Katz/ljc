use std::fs;
use ljc::lexer::{Token, Tokens};
use ljc::parser::Parser;
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

fn parser_snapshot_test(path: &Path) -> datatest_stable::Result<()> {
    let input = fs::read_to_string(path)?;
    if let Ok(program) = Parser::new(&input).parse() {
        let name = path.file_stem().unwrap().to_str().unwrap();
        insta::with_settings!({
            snapshot_path => "parser/snapshots",
            omit_expression => true,
        }, {
            insta::assert_snapshot!(name, program.to_string());
        });
    };
    Ok(()) // to satisfy return type check. insta fails when the output doesn't match
}

datatest_stable::harness! {
    { test = lexer_snapshot_test, root = "tests/snapshot/lexer", pattern = r"java" },
    { test = parser_snapshot_test, root = "tests/snapshot/parser", pattern = r"java" },
}
