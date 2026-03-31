use crate::lexer::error::{LexError, invalid_sequence, numeric_literal_error, invalid_escape};
use crate::lexer::tokens::Token;
use crate::lexer::util::{Radix, convert_to_int, is_whitespace};
use std::path::Path;
use std::{fs, io};

macro_rules! get {
    ( $x:expr ) => {
        match $x.peek() {
            Some(c) => c,
            None => return Ok(Token::EOF),
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

    pub fn next(&mut self) -> Result<Token, LexError> {
        loop {
            if is_whitespace(&get!(self)) {
                self.skip_whitespace();
            } else if self.accept_sequence("//") {
                self.eat_to_line_end();
            } else if self.accept_sequence("/*") {
                match self.eat_until("*/", EatMode::IncludeEnd) {
                    Err(_) => return invalid_sequence(self.line, self.column, "end of comment not found"),
                    _ => {},
                };
            } else {
                break;
            }
        }

        // separators
        if self.accept_sequence("...") {
            return Ok(Token::Ellipsis);
        }
        if self.accept_sequence("::") {
            return Ok(Token::DoubleColon);
        }
        let c = get!(self);
        if let Some(t) = match c {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            ';' => Some(Token::Semicolon),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '@' => Some(Token::At),
            _ => None,
        } {
            self.eat();
            return Ok(t);
        }

        if self.accept('\'') {
            if self.accept('\'') {
                return invalid_sequence(self.line, self.column, "Empty char literal");
            }
            let c = self.scan_char_literal()?;
            if self.accept('\'') {
                return Ok(Token::CharLiteral(c));
            }
            let column = self.column;
            self.eat_while(|tokens| {
                !matches!(tokens.peek(), Some('\r') | Some('\n') | Some('\''))
            }, EatMode::IncludeEnd);
            if self.accept('\'') {
                return invalid_sequence(self.line, column, "Too many characters in character literal");
            }
            return invalid_sequence(self.line, column, "Unclosed character literal");
        }
        if self.accept('"') {
            let s = self.scan_string_literal()?;
            if !self.accept('"') {
                return invalid_sequence(self.line, self.column, "Unterminated string literal");
            }
            return Ok(Token::StringLiteral(s));
        }

        if Self::is_operator_char(&c) {
            return Ok(self.scan_operator().unwrap());
        }

        if c == '0' {
            let numerical_token = match self.peek_next() {
                Some('x') | Some('X') => {
                    self.eat_n(2);
                    self.scan_number(Radix::Hexadecimal)
                }
                Some('b') | Some('B') => {
                    self.eat_n(2);
                    self.scan_number(Radix::Binary)
                }
                _ => self.scan_number(Radix::Octal),
            }?;
            return Ok(numerical_token);
        }
        if matches!(c, '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9') {
            let numerical_token = self.scan_number(Radix::Decimal)?;
            return Ok(numerical_token);
        }

        if c.is_alphabetic() {
            let identifier_or_kw = self.eat_while(|tokens| {
                match tokens.peek() {
                    None => false,
                    Some(c) => c.is_alphanumeric() || c == '_',
                }
            }, EatMode::IncludeEnd,).unwrap();
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

                // literals
                "null" => Token::NullLiteral,
                "true" => Token::BooleanLiteral(true),
                "false" => Token::BooleanLiteral(false),

                "_" => Token::Underscore,
                // TODO: add contextual keywords
                name => Token::Id(String::from(name)),
            };
            return Ok(token);
        }

        Ok(Token::EOF)
    }

    fn scan_char_literal(&mut self) -> Result<char, LexError> {
        if self.accept('\\') {
            if self.accept('b') { return Ok('\u{0008}') }
            if self.accept('s') { return Ok(' ') }
            if self.accept('t') { return Ok('\t') }
            if self.accept('n') { return Ok('\n') }
            if self.accept('f') { return Ok('\u{000C}') }
            if self.accept('r') { return Ok('\r') }
            if self.accept('"') { return Ok('"') }
            if self.accept('\'') { return Ok('\'') }
            if self.accept('\\') { return Ok('\\') }
            if let Some((n, len)) = Walk::while_holds(&self.input[self.pos..], |c| c.is_digit(8))
                .take(3)
                .zip(1..)
                .last() {
                let r = u8::from_str_radix(n, 8).unwrap() as char;
                self.eat_n(len);
                return Ok(r);
            }
            return invalid_escape(self.line, self.column);
        }
        match self.eat() {
            Some(c) => Ok(c),
            None => invalid_sequence(self.line, self.column, "Unclosed character literal")
        }
    }

    fn scan_string_literal(&mut self) -> Result<String, LexError> {
        let mut s = String::new();
        // let start = self.pos;
        loop {
            if self.peek() == Some('"'){
                break;
            }
            s.push(self.scan_char_literal()?);
        }

        Ok(s)
    }

    fn scan_number(&mut self, radix: Radix) -> Result<Token, LexError> {
        let radix: u32 = radix.into();
        match self.peek() {
            Some('_') => return numeric_literal_error(self.line, self.column, "Illegal underscore"),
            None => return numeric_literal_error(self.line, self.column, "Numbers must contain at least one digit"),
            _ => {}
        }
        let whole = self.eat_while(|tokens| {
            match tokens.peek() {
                Some(c) if c.is_digit(radix) => true,
                Some('_') => true,
                _ => false,
            }
        }, EatMode::IncludeEnd).unwrap();
        let value = convert_to_int(whole, radix).unwrap();
        if matches!(self.peek(), Some('_')) {
            return numeric_literal_error(self.line, self.column, "Illegal underscore");
        }
        match self.peek() {
            Some('l') | Some('L') => {
                self.eat();
                Ok(Token::LongLiteral(value))
            }
            _ => Ok(Token::IntegerLiteral(value)),
        }
    }

    fn is_operator_char(c: &char) -> bool {
        matches!(
            c,
            '=' | '>' | '<' | '!' | '~' | '?' | ':' | '&' | '|' | '+' | '-' | '*' | '/' | '^' | '%'
        )
    }

    fn scan_operator(&mut self) -> Option<Token> {
        let walk = Walk::new(&self.input[self.pos..]);

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
                return is_whitespace(&c);
            }
            false
        }, EatMode::IncludeEnd);
    }

    fn eat_to_line_end(&mut self) {
        self.eat_while(|tokens| {
            if let Some(c) = tokens.peek() {
                return match c {
                    '\r' if tokens.next_is('\n') => true,
                    '\n' | '\r' => {
                        false
                    }
                    _ => true,
                };
            }
            false
        }, EatMode::IncludeEnd);
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

    fn accept(&mut self, desired: char) -> bool {
        match self.peek() {
            Some(s) if s == desired => {
                self.eat();
                true
            }
            _ => false,
        }
    }

    fn accept_sequence(&mut self, sequence: &str) -> bool {
        if sequence.is_empty() {
            return true;
        }
        let sequence_len = sequence.chars().count();
        let mut walk = Walk::new(&self.input[self.pos..]).skip(sequence_len - 1);
        match walk.next() {
            Some(s) if s == sequence => {
                self.eat_n(sequence_len);
                true
            }
            _ => false,
        }
    }

    fn eat(&mut self) -> Option<char> {
        let current = self.peek()?;
        self.column += 1;
        let next_is_lf = self.next_is('\n');
        if current == '\n' || (current == '\r' && !next_is_lf) {
            self.next_line()
        }
        self.pos += current.len_utf8();
        Some(current)
    }

    fn eat_n(&mut self, mut n: usize) -> Option<&str> {
        let mut len = 0;
        while n > 0 {
            len += self.eat()?.len_utf8();
            n -= 1;
        }
        Some(&self.input[self.pos - len..self.pos])
    }

    /// Eat input characters until given sequence is encountered.
    ///
    /// # Arguments
    ///
    /// * `sequence`: the sequence to look for
    /// * `eat_mode`: controls the string returned.</br>
    ///               - `IncludeEnd`: the returned string includes the terminating sequence</br>
    ///               - `NoEnd`: the returned string does not include the terminating sequence</br>
    ///               In any case, the terminating sequence is eaten as well.
    ///
    fn eat_until(&mut self, sequence: &str, eat_mode: EatMode) -> Result<&str, ()> {
        if sequence.is_empty() {
            return Ok(&self.input[self.pos..self.pos]);
        }
        if let Some(i) = self.input[self.pos..].find(&sequence[0..]) {
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
        F: Fn(&mut Self) -> bool,
    {
        let mut last_char = match self.peek() {
            Some(c) => c,
            None => return None,
        };
        let start = self.pos;
        while let Some(c) = self.peek() {
            last_char = c;
            if !predicate(self) {
                break;
            }
            self.eat();
        }
        let end = if eat_mode.include_end() { self.pos } else { self.pos - last_char.len_utf8() };
        Some(&self.input[start..end])
    }
}

#[allow(dead_code)]
enum EatMode {
    IncludeEnd,
    NoEnd,
}

impl EatMode {
    pub fn include_end(&self) -> bool {
        match self {
            EatMode::IncludeEnd => true,
            EatMode::NoEnd => false,
        }
    }
}

struct Walk<'a> {
    s: &'a str,
    pos: usize,
    predicate: fn(&char) -> bool,
}

impl<'a> Walk<'a> {
    pub fn new(tokens: &'a str) -> Self {
        Self { s: tokens, pos: 0, predicate: |_c| true }
    }

    pub fn while_holds(tokens: &'a str, predicate: fn(&char) -> bool) -> Self{
        Self {s: tokens, pos: 0, predicate }
    }
}

impl<'a> Iterator for Walk<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.s[self.pos..].chars().next()?;
        if !(self.predicate)(&c) {
            return None
        }
        self.pos += c.len_utf8();
        Some(&self.s[..self.pos])
    }
}
