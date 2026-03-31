use std::num::ParseIntError;

pub enum Whitespace {
    SPACE,
    TAB,
    FF,
    LF,
    CR
}

impl TryFrom<&char> for Whitespace {
    type Error = ();

    fn try_from(c: &char) -> Result<Self, Self::Error> {
        match c {
            ' '  => Ok(Whitespace::SPACE),
            '\t' => Ok(Whitespace::TAB),
            '\x0c' => Ok(Whitespace::FF),
            '\n' => Ok(Whitespace::LF),
            '\r' => Ok(Whitespace::CR),
            _ => Err(()),
        }
    }
}

pub fn is_whitespace(char: &char) -> bool {
    Whitespace::try_from(char).is_ok()
}

pub enum Radix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Into<u8> for Radix {
    fn into(self) -> u8 {
        match self {
            Radix::Binary => 2,
            Radix::Octal => 8,
            Radix::Decimal => 10,
            Radix::Hexadecimal => 16,
        }
    }
}

impl Into<u32> for Radix {
    fn into(self) -> u32 {
        <Radix as Into<u8>>::into(self) as u32
    }
}

pub fn convert_to_int(s: &str, radix: u32) -> Result<u64, ParseIntError> {
    let cleaned: String = s.chars().filter(|&c| {c != '_'}).collect();
    u64::from_str_radix(&cleaned, radix)
}