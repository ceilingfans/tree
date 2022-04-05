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
    /// Whitespace
    Whitespace,
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
    fn eat_decimal_digits(&mut self) -> String {
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

    fn eat_number(&mut self) -> String {
        if self.peek_first() == '0' {
            match self.peek_second() {
                'x' | 'X' => {
                    self.advance();
                    self.advance();
                    return self.eat_hexadecimal_digits()
                },
                'b' | 'B' => {
                    self.advance();
                    self.advance();
                    return self.eat_binary_digits()
                },
                // allow underscores as we allow underscores in the number literal
                // as we allow them in eat_x_digits methods for readability
                '0'..='9' | '_' => self.eat_decimal_digits(),
                // TODO: actual error message
                _ => panic!("Unexpected character after 0"),
            }
        } else {
            self.eat_decimal_digits()
        }
    }

    /// Gobbles up a comment
    ///
    /// Regular comments start with `two forward slashes // and end with a newline`
    fn eat_comment(&mut self) {
        self.eat_while(|c| c != '\n');
    }

    /// Gobbles up a multiline comment
    ///
    /// Multiline comments start with `/*`, and end with `*/`
    fn eat_multiline_comment(&mut self) {
        while let Some(c) = self.advance() {
            match c {
                '*' => {
                    if self.peek_first() == '/' {
                        self.advance();
                        break;
                    }
                },
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eat_decimal_digits() {
        let mut cursor = Cursor::new("123_456 ");
        assert_eq!(cursor.eat_decimal_digits(), "123456");
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

    #[test]
    fn test_eat_binary_digits() {
        let mut cursor = Cursor::new("0b_101_01 ");

        // move two ahead to eat '0b'
        cursor.advance();
        cursor.advance();

        assert_eq!(cursor.eat_binary_digits(), "10101");
        assert_eq!(cursor.advance(), Some(' '));
    }

    #[test]
    fn test_eat_number() {
        let mut binary_cursor = Cursor::new("0b10101");
        assert_eq!(binary_cursor.eat_number(), "10101");
        assert!(binary_cursor.is_eof());

        let mut hexadecimal_cursor = Cursor::new("0xAf123D");
        assert_eq!(hexadecimal_cursor.eat_number(), "Af123D");
        assert!(hexadecimal_cursor.is_eof());

        let mut decimal_cursor = Cursor::new("123_456");
        assert_eq!(decimal_cursor.eat_number(), "123456");
        assert!(decimal_cursor.is_eof());
    }

    #[test]
    #[should_panic]
    fn test_eat_number_fail() {
        let mut cursor = Cursor::new("0p123");
        cursor.eat_number();
    }

    #[test]
    fn test_eat_comment() {
        let mut cursor = Cursor::new("// This is a comment\nballs");
        cursor.eat_comment();
        assert_eq!(cursor.advance(), Some('\n'));
    }

    #[test]
    fn test_eat_multiline_comment() {
        let src = "/* this is a multiline comment\n\
                   with multiple lines\n\
                   and stuff */\n\
                   balls";

        let mut cursor = Cursor::new(src);
        cursor.eat_multiline_comment();
        assert_eq!(cursor.advance(), Some('\n'));
    }

    #[test]
    fn test_failing_multiline_comment() {
        let src = "/* obama obama *l";

        let mut cursor = Cursor::new(src);
        cursor.eat_multiline_comment();
        assert_eq!(cursor.advance(), None);
    }
}