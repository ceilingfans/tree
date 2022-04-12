//! The lexer for tree lang (heavily inspired by the rust compiler's `rustc_lexer` crate)
extern crate unicode_xid;

use unicode_xid::UnicodeXID;
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
    Ident {
        keyword: bool,
    },
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
                }
                c @ '0'..='9' => {
                    ret.push(c);
                    self.advance();
                }
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
                }
                c @ ('0'..='9' | 'a'..='f' | 'A'..='F') => {
                    ret.push(c);
                    self.advance();
                }
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
                }
                c @ '0'..='1' => {
                    ret.push(c);
                    self.advance();
                }
                _ => {
                    break;
                }
            }
        }

        ret
    }

    /// Gobbles up the number literal (can only be base 2, 10 and 16) and returns it as a `String`.
    ///
    /// Panics
    ///
    /// - If the number literal attempts to specify a base that is invalid,
    /// e.g. `0p123`
    fn eat_number(&mut self) -> (String, Base) {
        if self.peek_first() == '0' {
            match self.peek_second() {
                'x' | 'X' => {
                    self.advance();
                    self.advance();
                    (self.eat_hexadecimal_digits(), Base::Hexadecimal)
                }
                'b' | 'B' => {
                    self.advance();
                    self.advance();
                    (self.eat_binary_digits(), Base::Binary)
                }
                // allow underscores as we allow underscores in the number literal
                // as we allow them in eat_x_digits methods for readability
                '0'..='9' | '_' => (self.eat_decimal_digits(), Base::Decimal),
                // TODO: actual error message
                _ => panic!("Unexpected character after 0"),
            }
        } else {
            (self.eat_decimal_digits(), Base::Decimal)
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
        let mut depth = 0;
        while let Some(c) = self.advance() {
            if c == '*' && self.peek_first() == '/' {
                depth -= 1;
                self.advance();
                if depth == 0 {
                    break;
                }
            } else if c == '/' && self.peek_first() == '*' {
                depth += 1;
                self.advance();
            }
        }
    }

    /// Gobbles up an identifier, and returns it as a `String`,
    /// Identifier naming rules follow Rusts naming rules.
    ///
    /// Panics
    ///
    /// - If the identifier has a bad start
    fn eat_ident(&mut self) -> String {
        if !is_xid_start(self.peek_first()){
            panic!("bad identifier start"); // TODO: actual error message
        }

        // move past ident start
        let mut ret = String::from(self.advance().unwrap());
        ret.push_str(self.eat_while(is_xid_continue).as_str());

        ret
    }

    fn eat_double_quoted_string(&mut self) -> String {
        let mut ret = String::new();

        self.advance(); // eat start quote

        loop {
            match self.peek_first() {
                '\\' => {
                    self.advance(); // eat escape backslash
                    match self.peek_first() {
                        '\\' => {
                            ret.push('\\');
                            self.advance();
                        },
                        'n' => {
                            ret.push('\n');
                            self.advance();
                        },
                        'r' => {
                            ret.push('\r');
                            self.advance();
                        },
                        't' => {
                            ret.push('\t');
                            self.advance();
                        },
                        '"' => {
                            ret.push('"');
                            self.advance();
                        },
                        '\'' => {
                            ret.push('\'');
                            self.advance();
                        },
                        _ => {
                            panic!("unexpected escape sequence"); // TODO: actual error message
                        }
                    }
                },
                '"' => {
                    self.advance();
                    break;
                },
                EOF => {
                    panic!("unterminated string"); // TODO: actual error message
                },
                _ => {
                    ret.push(self.advance().unwrap());
                }
            }
        }

        ret
    }
}

fn is_xid_start(c: char) -> bool {
    c == '_' || UnicodeXID::is_xid_start(c)
}

fn is_xid_continue(c: char) -> bool {
    UnicodeXID::is_xid_continue(c)
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
        let binary_expected = (String::from("10101"), Base::Binary);
        assert_eq!(binary_cursor.eat_number(), binary_expected);
        assert!(binary_cursor.is_eof());

        let mut hexadecimal_cursor = Cursor::new("0xAf123D");
        let hexadecimal_expected = (String::from("Af123D"), Base::Hexadecimal);
        assert_eq!(hexadecimal_cursor.eat_number(), hexadecimal_expected);
        assert!(hexadecimal_cursor.is_eof());

        let mut decimal_cursor = Cursor::new("123_456");
        let decimal_expected = (String::from("123456"), Base::Decimal);
        assert_eq!(decimal_cursor.eat_number(), decimal_expected);
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
    fn test_nested_multiline_comment() {
        let src = "/* /* nested */ */a";

        let mut cursor = Cursor::new(src);
        cursor.eat_multiline_comment();
        assert_eq!(cursor.advance(), Some('a'));
    }

    #[test]
    fn test_failing_multiline_comment() {
        let src = "/* obama obama *l";

        let mut cursor = Cursor::new(src);
        cursor.eat_multiline_comment();
        assert_eq!(cursor.advance(), None);
    }

    #[test]
    fn test_eat_ident() {
        let mut cursor = Cursor::new("snake_case camelCase PascalCase SCREAMING_CASE _private");

        assert_eq!(cursor.eat_ident(), "snake_case");
        cursor.advance();

        assert_eq!(cursor.eat_ident(), "camelCase");
        cursor.advance();

        assert_eq!(cursor.eat_ident(), "PascalCase");
        cursor.advance();

        assert_eq!(cursor.eat_ident(), "SCREAMING_CASE");
        cursor.advance();

        assert_eq!(cursor.eat_ident(), "_private");
        assert!(cursor.is_eof());
    }

    #[test]
    fn test_eat_ident_wonky_names() {
        let mut cursor = Cursor::new("猫数 Кот");

        assert_eq!(cursor.eat_ident(), "猫数");
        cursor.advance();

        assert_eq!(cursor.eat_ident(), "Кот");
        assert!(cursor.is_eof());
    }

    #[test]
    #[should_panic]
    fn test_eat_ident_fail() {
        let mut cursor = Cursor::new("10Zed");

        cursor.eat_ident();
    }

    #[test]
    fn test_eat_double_quoted_string() {
        let mut cursor = Cursor::new(r#""a b c\"""#);

        assert_eq!(cursor.eat_double_quoted_string(), "a b c\"");
        assert!(cursor.is_eof());
    }
}
