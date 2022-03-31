//! The lexer for tree lang (heavily inspired by the rust compiler's `rustc_lexer` crate)
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
    Number,
    String,
    Bool,
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
