use std::str::Chars;

const EOF: char = '\0';

/// A Peekable iterator over a character sequence
///
/// To peek, use the `peek_first` and `peek_second` methods,
/// the position can be moved forwards using the `advance` method
pub(crate) struct Cursor<'a> {
    chars: Chars<'a>,
    initial_len: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Cursor {
            chars: input.chars(),
            initial_len: input.len()
        }
    }

    /// Returns how many chars have been eaten
    pub(crate) fn consumed(&self) -> usize {
        self.initial_len - self.chars.as_str().len()
    }

    /// Resets the amount of eaten chars to 0
    pub(crate) fn reset_consumed(&mut self) {
        self.initial_len = self.chars.as_str().len();
    }

    /// Returns whether the iterator is at the end of file
    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    /// Peeks at the next char in the iterator,
    /// returns an EOF char if the position cannot be found
    pub(crate) fn peek_first(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF)
    }

    /// Peeks at the second next char in the iterator,
    /// returns an EOF char if the position cannot be found
    pub(crate) fn peek_second(&self) -> char {
        let mut chars = self.chars.clone();
        chars.next();

        chars.next().unwrap_or(EOF)
    }

    /// Advances the iterator to move forward
    pub(crate) fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }
}