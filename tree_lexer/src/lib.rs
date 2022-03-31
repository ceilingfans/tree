//! The lexer for tree lang (heavily inspired by the rust compiler's `rustc_lexer` crate)
mod cursor;

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

pub enum LiteralKind {
    Ident {
        keyword: bool
    },
    Number,
    String,
    Bool
}

pub struct Location {
    pub line: usize,
    pub column: usize,
}

pub struct Token {
    pub kind: TokenKind,
    pub literal: String,
    pub location: Location,
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