use std::cmp::max;
use crate::lexer::error::{invalid_sequence, LexError};
use crate::lexer::tokens::Token;
use crate::lexer::util::is_whitespace;
use std::{fs, io};
use std::path::Path;

macro_rules! get {
    ( $x:expr ) => {
        match $x.peek() {
            Some(c) => c,
            None => return Ok(None),
        }
    };
}

pub fn lex_single_file(file_path: &Path) -> Result<Tokens, io::Error> {
    let input = fs::read_to_string(file_path)?;
    Ok(Tokens::new(input))
}

pub struct Tokens {
    input: String,
    pos: usize,
    line: usize,
    column: usize,
}

impl Tokens {
    pub fn new(input: String) -> Self {
        Self {
            input,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next(&mut self) -> Result<Option<Token>, LexError> {
        let mut c = get!(self);
        loop {
            if is_whitespace(&c) {
                self.skip_whitespace();
                c = get!(self);
            }
            else if c == '/'
                && let Some(next) = self.peek_next()
            {
                if next == '/' {
                    self.eat_to_line_end();
                    c = get!(self);
                }
                else if next == '*' {
                    match self.eat_until("*/", EatMode::NoEnds) {
                        Err(_) => {
                            return invalid_sequence(self.line, self.column, "end of comment not found");
                        }
                        _ => {}
                    };
                    c = get!(self);
                }
                else {
                    break;
                }
            }
            else {
                break;
            }
        }

        // separators
        if let Some(t) = match c {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            ';' => Some(Token::Semicolon),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot), // TODO: handle ellipsis
            '@' => Some(Token::At),
            ':' if self.next_is(':') => {
                self.eat(); // need to dispose of two characters
                Some(Token::DoubleColon)
            }
            _ => None,
        } {
            self.eat();
            return Ok(Some(t));
        }

        if c == '\'' {
            // FIXME: need to only look until end of line
            return match self.eat_until("'", EatMode::NoEnds) {
                Ok(s) if s.len() == 0 => {
                    invalid_sequence(self.line, self.column, "Empty char literal")
                }
                // 2 characters is the char itself and the closing quote
                Ok(s) if s.len() > 1 => invalid_sequence(
                    self.line,
                    self.column,
                    "Too many characters in character literal",
                ),
                Ok(s) => Ok(Some(Token::CharLiteral(s.chars().next().unwrap()))),
                Err(()) => invalid_sequence(self.line, self.column, "Unclosed character literal"),
            };
        }
        if c == '"' {
            // FIXME: need to only look until end of line
            return match self.eat_until("\"", EatMode::NoEnds) {
                Ok(s) => Ok(Some(Token::StringLiteral(String::from(s)))),
                Err(()) => invalid_sequence(self.line, self.column, "Unclosed string literal"),
            };
        }

        if Self::is_operator_char(&mut c) {
            return Ok(Some(self.scan_operator().unwrap()));
        }

        if c.is_alphabetic() {
            let identifier_or_kw = self.eat_while(|tokens| {
                match  tokens.peek() {
                    None => false,
                    Some(c) => c.is_alphanumeric() || c == '_'
                }
            }, EatMode::BothEnds).unwrap();
            let token = match identifier_or_kw {
                // keywords
                "abstract" => Token::Abstract,
                "assert" => Token::Assert,
                "boolean" => Token::Boolean,
                "break" => Token::Break,
                "byte" => Token::Byte,
                "case" => Token::Case,
                "catch" => Token::Catch,
                "char" => Token::Char,
                "class" => Token::Class,
                "const" => Token::Const,
                "continue" => Token::Continue,
                "default" => Token::Default,
                "do" => Token::Do,
                "double" => Token::Double,
                "else" => Token::Else,
                "enum" => Token::Enum,
                "extends" => Token::Extends,
                "final" => Token::Final,
                "finally" => Token::Finally,
                "float" => Token::Float,
                "for" => Token::For,
                "if" => Token::If,
                "goto" => Token::Goto,
                "implements" => Token::Implements,
                "import" => Token::Import,
                "instanceof" => Token::Instanceof,
                "int" => Token::Int,
                "interface" => Token::Interface,
                "long" => Token::Long,
                "native" => Token::Native,
                "new" => Token::New,
                "package" => Token::Package,
                "private" => Token::Private,
                "protected" => Token::Protected,
                "public" => Token::Public,
                "return" => Token::Return,
                "short" => Token::Short,
                "static" => Token::Static,
                "strictfp" => Token::Strictfp,
                "super" => Token::Super,
                "switch" => Token::Switch,
                "synchronized" => Token::Synchronized,
                "this" => Token::This,
                "throw" => Token::Throw,
                "throws" => Token::Throws,
                "transient" => Token::Transient,
                "try" => Token::Try,
                "void" => Token::Void,
                "volatile" => Token::Volatile,
                "while" => Token::While,

                // contextual keywords
                "exports" => Token::Exports,
                "module" => Token::Module,
                "nonSealed" => Token::NonSealed,
                "open" => Token::Open,
                "opens" => Token::Opens,
                "permits" => Token::Permits,
                "provides" => Token::Provides,
                "record" => Token::Record,
                "requires" => Token::Requires,
                "sealed" => Token::Sealed,
                "to" => Token::To,
                "transitive" => Token::Transitive,
                "uses" => Token::Uses,
                "var" => Token::Var,
                "when" => Token::When,
                "with" => Token::With,
                "yield" => Token::Yield,

                "_" => Token::Underscore,
                // TODO: add contextual keywords
                name => Token::Id(String::from(name))
            };
            return Ok(Some(token));
        }

        Ok(None)
    }

    fn is_operator_char(c: &mut char) -> bool {
        matches!(c,
        '=' | '>' | '<' | '!' | '~' | '?' | ':' | '&' | '|' | '+' | '-' | '*' | '/' | '^' | '%')
    }

    fn scan_operator(&mut self) -> Option<Token> {
        let walk = Walk::new(&mut self.input[self.pos..]);

        let (token, len) = walk.into_iter()
            .map(|s| Self::take_operator(s))
            .take_while(|t| t.is_some())
            .last()
            .flatten()?;
        self.pos += len;
        Some(token)
    }

    fn take_operator(s: &str) -> Option<(Token, usize)> {
        match s {
            "=" => Some((Token::Assign, 1)),
            ">" => Some((Token::GreaterThan, 1)),
            "<" => Some((Token::LessThan, 1)),
            "!" => Some((Token::ExclamationMark, 1)),
            "~" => Some((Token::Tilde, 1)),
            "?" => Some((Token::QuestionMark, 1)),
            ":" => Some((Token::Colon, 1)),
            "->" => Some((Token::Arrow, 2)),
            "==" => Some((Token::Equals, 2)),
            ">=" => Some((Token::GreaterThanOrEquals, 2)),
            "<=" => Some((Token::LessThanOrEquals, 2)),
            "!=" => Some((Token::NotEquals, 2)),
            "&&" => Some((Token::LogicalAnd, 2)),
            "||" => Some((Token::LogicalOr, 2)),
            "++" => Some((Token::Increment, 2)),
            "--" => Some((Token::Decrement, 2)),
            "+" => Some((Token::Plus, 1)),
            "-" => Some((Token::Minus, 1)),
            "*" => Some((Token::Multiply, 1)),
            "/" => Some((Token::Divide, 1)),
            "&" => Some((Token::BitwiseAnd, 1)),
            "|" => Some((Token::BitwiseOr, 1)),
            "^" => Some((Token::BitwiseXor, 1)),
            "%" => Some((Token::Modulo, 1)),
            "<<" => Some((Token::LeftShift, 2)),
            ">>" => Some((Token::SignedRightShift, 2)),
            ">>>" => Some((Token::UnsignedRightShift, 3)),
            "+=" => Some((Token::AddAssign, 2)),
            "-=" => Some((Token::SubAssign, 2)),
            "*=" => Some((Token::MulAssign, 2)),
            "/=" => Some((Token::DivAssign, 2)),
            "&=" => Some((Token::AndAssign, 2)),
            "|=" => Some((Token::OrAssign, 2)),
            "^=" => Some((Token::XorAssign, 2)),
            "%=" => Some((Token::ModAssign, 2)),
            "<<=" => Some((Token::LeftShiftAssign, 3)),
            ">>=" => Some((Token::SignedRightShiftAssign, 3)),
            ">>>=" => Some((Token::UnsignedRightShiftAssign, 4)),
            _ => None,
        }
    }

    fn skip_whitespace(&mut self) {
        self.eat_while(|tokens| {
            if let Some(c) = tokens.peek() {
                match c {
                    '\r' if tokens.next_is('\n') => {} // will be handled in next step by below line
                    '\n' | '\r' => {
                        // singular CR not followed by LF (above guard) counts as a line terminator
                        tokens.next_line();
                    }
                    _ => {}
                }
                return is_whitespace(&c);
            }
            false
        }, EatMode::NoEnds);
    }

    fn eat_to_line_end(&mut self) {
        self.eat_while(|tokens| {
            if let Some(c) = tokens.peek() {
                return match c {
                    '\r' if tokens.next_is('\n') => true,
                    '\n' | '\r' => {
                        tokens.next_line();
                        tokens.eat();
                        false
                    }
                    _ => true,
                };
            }
            false
        }, EatMode::NoEnds);
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    fn next_is(&mut self, c: char) -> bool {
        self.peek_next() == Some(c)
    }

    fn peek(&self) -> Option<char> {
        self.peek_n(0)
    }

    fn peek_next(&self) -> Option<char> {
        self.peek_n(1)
    }

    fn peek_n(&self, mut n: usize) -> Option<char> {
        let mut chars = self.input[self.pos..].chars();
        while n > 0 {
            chars.next()?;
            n -= 1;
        }
        chars.next()
    }

    fn eat(&mut self) -> Option<char> {
        let c = self.peek()?;
        if c == '\n' {
            self.line += 1;
        }
        self.pos += c.len_utf8();
        Some(c)
    }

    fn eat_until(&mut self, sequence: &str, eat_mode: EatMode) -> Result<&str, ()> {
        if !eat_mode.include_current() {
            self.eat();
        }
        if let Some(i) = self.input[self.pos..].find(sequence) {
            let slice_len = i
                + (if eat_mode.include_end() {
                    sequence.len()
                } else {
                    0
                });
            let advance_len = i + sequence.len();
            let s = &self.input[self.pos..self.pos + slice_len];
            self.pos += advance_len;
            Ok(s)
        } else {
            Err(())
        }
    }

    fn eat_while<F>(&mut self, predicate: F, eat_mode: EatMode) -> Option<&str>
    where
        F: Fn(&mut Self) -> bool
    {
        let mut last_char = match self.peek() {
            Some(c) => c,
            None => return None
        };
        let start =  if eat_mode.include_current() {self.pos} else {self.pos + last_char.len_utf8()};
        while let Some(c) = self.peek() {
            last_char = c;
            if !predicate(self) {
                break;
            }
            self.eat();
        }
        let end = if eat_mode.include_end() {self.pos} else {self.pos - last_char.len_utf8()};
        Some(&self.input[start..max(start, end)])
    }
}

#[allow(dead_code)]
enum EatMode {
    StartOnly,
    NoEnds,
    BothEnds,
    EndOnly,
}

impl EatMode {
    pub fn include_current(&self) -> bool {
        match self {
            EatMode::StartOnly | EatMode::BothEnds => true,
            _ => false,
        }
    }

    pub fn include_end(&self) -> bool {
        match self {
            EatMode::EndOnly | EatMode::BothEnds => true,
            _ => false,
        }
    }
}

struct Walk<'a> {
    s: &'a str,
    pos: usize,
}

impl<'a> Walk<'a> {
    pub fn new(tokens: &'a mut str) -> Self {
        Self { s: tokens, pos: 0 }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos += &self.s[self.pos..].chars().next()?.len_utf8();
        Some(&self.s[..self.pos])
    }
}