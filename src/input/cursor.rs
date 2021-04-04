/// A specific position inside a `Reader`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cursor {
    byte_offset: usize,
    char_offset: usize,
    line: usize,
    column: usize,
}

impl Cursor {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Builds a new `Cursor` with the specified data.
    pub(in crate::input) fn new(
        byte_offset: usize,
        char_offset: usize,
        line: usize,
        column: usize,
    ) -> Cursor {
        Cursor {
            byte_offset,
            char_offset,
            line,
            column,
        }
    }

    // GETTERS ----------------------------------------------------------------

    /// The position of the `Cursor` in bytes.
    #[inline]
    pub fn byte_offset(&self) -> usize {
        self.byte_offset
    }

    /// The position of the `Cursor` in characters.
    #[inline]
    pub fn char_offset(&self) -> usize {
        self.char_offset
    }

    /// The line number in which the `Cursor` is placed.
    /// It starts at line 1.
    #[inline]
    pub fn line(&self) -> usize {
        self.line
    }

    /// The column number in which the `Cursor` is placed.
    /// It starts at column 1.
    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }
}
