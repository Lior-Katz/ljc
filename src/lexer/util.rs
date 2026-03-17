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