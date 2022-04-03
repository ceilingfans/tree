//! The lexer for tree lang (heavily inspired by the rust compiler's `rustc_lexer` crate)
use crate::cursor::{Cursor, EOF};

mod cursor;

/// Represents the kind of token
#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenKind {
    // Single character tokens
    /// `+`
    Add,
    /// `-`
    Minus,
    /// `/`
    Divide,
    /// `*`
    Multiply,
    /// `%`
    Modulo,
    /// `.`
    Dot,
    /// `,`
    Comma,
    /// `;`
    Semicolon,
    /// `:`
    Colon,
    /// `(`
    LeftParen,
    /// `)`
    RightParen,
    /// `{`
    LeftBrace,
    /// `}`
    RightBrace,
    /// `[`
    LeftBracket,
    /// `]`
    RightBracket,
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `&`
    BitwiseAnd,
    /// `|`
    BitwiseOr,
    /// `!`
    Not,
    /// `=`
    Assign,

    // Multi-character tokens
    /// Literals
    ///
    /// Used to represent `identifiers`, `numbers`, `strings` and `boolean` values
    Literal(LiteralKind),
    /// `<=`
    LessThanEqual,
    /// `>=`
    GreaterThanEqual,
    /// `==`
    Equal,
    /// `!=`
    NotEqual,
    /// `&&`
    And,
    /// `||`
    Or,
}

/// Represents the type of literal,
/// used to differentiate between `identifiers`, `numbers`, `strings` and `boolean` values
#[derive(Debug, PartialEq, PartialOrd)]
pub enum LiteralKind {
    /// An `identifier`, used to name things.
    /// Also used for keywords
    Ident { keyword: bool },
    Number(Base),
    String,
    Bool,
}

/// Represents the base of a number literal
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Base {
    Hexadecimal,
    Decimal,
    Binary,
}

/// Represents the location the first character of the token is at
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

/// Represents a token use for parsing
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub location: Location,
    /// The length of the token
    pub length: usize,
}

impl Token {
    fn new(kind: TokenKind, literal: String, location: Location, length: usize) -> Token {
        Token {
            kind,
            literal,
            location,
            length,
        }
    }
}

impl Cursor<'_> {
    /// Gobbles up the number literal and returns it as a `String`.
    /// Ignores `_` characters to allow underscores in the number literal for readability.
    fn eat_digits(&mut self) -> String {
        let mut ret = String::new();

        loop {
            match self.peek_first() {
                '_' => {
                    self.advance();
                },
                c @ '0'..='9' => {
                    ret.push(c);
                    self.advance();
                },
                _ => {
                    break;
                }
            }
        }

        ret
    }

    /// Gobbles up case-insensitive hexadecimal digits and returns them as a `String`.
    /// Ignores `_` characters to allow underscores in the number literal for readability.
    fn eat_hexadecimal_digits(&mut self) -> String {
        let mut ret = String::new();

        loop {
            match self.peek_first() {
                '_' => {
                    self.advance();
                },
                c @ ('0'..='9' | 'a'..='f' | 'A'..='F') => {
                    ret.push(c);
                    self.advance();
                },
                _ => {
                    break;
                }
            }
        }

        ret
    }

    /// Gobbles up binary digits and returns them as a `String`.
    /// Ignores `_` characters to allow underscores in the number literal for readability.
    fn eat_binary_digits(&mut self) -> String {
        let mut ret = String::new();

        loop {
            match self.peek_first() {
                '_' => {
                    self.advance();
                },
                c @ '0'..='1' => {
                    ret.push(c);
                    self.advance();
                },
                _ => {
                    break;
                }
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat_digits() {
        let mut cursor = Cursor::new("123_456 ");
        assert_eq!(cursor.eat_digits(), "123456");
        assert_eq!(cursor.advance(), Some(' '));
    }

    #[test]
    fn test_eat_hexadecimal_digits() {
        let mut cursor = Cursor::new("0xAf_123_D ");

        // move two ahead to eat '0x'
        cursor.advance();
        cursor.advance();

        assert_eq!(cursor.eat_hexadecimal_digits(), "Af123D");
        assert_eq!(cursor.advance(), Some(' '));
    }
}