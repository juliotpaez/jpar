use std::ops::RangeInclusive;

use bytecount::num_chars;
use memchr::Memchr;

pub use cursor::*;
pub use span::*;

mod cursor;
mod span;

/// A `String` reader that moves a cursor the reader updated.
#[derive(Debug, Clone)]
pub struct Reader<'a, C = ()> {
    content: &'a str,
    cursor: Cursor,
    context: Option<C>,
}

impl<'a> Reader<'a> {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Create a new `Reader` with the specified `content`.
    pub fn new(content: &'a str) -> Reader<'a, ()> {
        Reader {
            content,
            cursor: Cursor::new(0, 0, 1, 1),
            context: None,
        }
    }
}

impl<'a, C> Reader<'a, C> {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Create a new `Reader` with the specified `content` and `context`.
    pub fn new_with_context(content: &'a str, context: C) -> Reader<'a, C> {
        Reader {
            content,
            cursor: Cursor::new(0, 0, 1, 1),
            context: Some(context),
        }
    }

    // GETTERS ----------------------------------------------------------------

    /// The associated context of the `Reader` if there's any.
    pub fn context(&self) -> &Option<C> {
        &self.context
    }

    /// The content of the `Reader`.
    pub fn content(&self) -> &'a str {
        self.content
    }

    /// The position of the `Reader` in bytes.
    pub fn byte_offset(&self) -> usize {
        self.cursor.byte_offset()
    }

    /// The position of the `Cursor` in characters.
    /// It starts at char 0.
    pub fn char_offset(&self) -> usize {
        self.cursor.char_offset()
    }

    /// The line number of the current position.
    /// It starts at line 1.
    pub fn line(&self) -> usize {
        self.cursor.line()
    }

    /// The column number of the current position.
    /// It starts at column 1.
    pub fn column(&self) -> usize {
        self.cursor.column()
    }

    /// The remaining content as an `Slice`.
    pub fn remaining_content(&self) -> &'a str {
        &self.content[self.cursor.byte_offset()..]
    }

    /// The length in bytes of the content that is not already read.
    pub fn remaining_length(&self) -> usize {
        self.content.len() - self.cursor.byte_offset()
    }

    /// The length in characters of the content that is not already read.
    pub fn remaining_char_length(&self) -> usize {
        num_chars(self.remaining_content().as_bytes())
    }

    /// Returns an empty `Span` located at the current position.
    pub fn span_at_offset(&self) -> Span {
        let cursor = self.cursor.clone();
        Span::new(self.content.clone(), cursor.clone(), cursor)
    }

    /// Whether the reader is placed at the end of the input or not.
    pub fn is_end(&self) -> bool {
        self.cursor.byte_offset() >= self.content.len()
    }

    // METHODS ----------------------------------------------------------------

    /// Consumes the next character if present moving the start index forward.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("test");
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('e'));
    /// assert_eq!(reader.read(), Some('s'));
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), None);
    /// ```
    pub fn read(&mut self) -> Option<char> {
        match self.peek() {
            Some(v) => {
                self.consume(v.len_utf8());
                Some(v)
            }
            None => None,
        }
    }

    /// Checks whether the reader continues with one or more of the characters specified by `interval`.
    /// This method does not consume the reader.
    ///
    /// **Note**: this method requires `interval` be sorted.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("this test");
    ///
    /// let result = reader.read_while(|i,c| ('a'..='z').contains(&c));
    /// assert_eq!(result, "this");
    ///
    /// assert_eq!(reader.read(), Some(' '));
    ///
    /// let result = reader.read_while(|i,c| ('0'..='9').contains(&c));
    /// assert_eq!(result, "");
    ///
    /// let result = reader.read_while(|i,c| ('a'..='z').contains(&c));
    /// assert_eq!(result, "test");
    /// ```
    pub fn read_while<F>(&mut self, condition: F) -> &'a str
    where
        F: FnMut(usize, char) -> bool,
    {
        let result = self.peek_while(condition);
        self.consume(result.len());
        result
    }

    /// Gets the next character if present. This method does not consume the character.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("test");
    /// assert_eq!(reader.peek(), Some('t'));
    /// assert_eq!(reader.peek(), Some('t'));
    /// assert_eq!(reader.read(), Some('t'));
    ///
    /// assert_eq!(reader.peek(), Some('e'));
    /// assert_eq!(reader.read(), Some('e'));
    /// assert_eq!(reader.read(), Some('s'));
    /// assert_eq!(reader.read(), Some('t'));
    ///
    /// assert_eq!(reader.peek(), None);
    /// ```
    pub fn peek(&self) -> Option<char> {
        let remaining = self.remaining_content();
        remaining.chars().next()
    }

    /// Checks whether the reader continues with one or more of the characters specified by `interval`.
    /// This method does not consume the reader.
    ///
    /// **Note**: this method requires `interval` be sorted.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("this test");
    /// assert_eq!(reader.byte_offset(), 0);
    ///
    /// let result = reader.peek_while(|i,c| ('a'..='z').contains(&c));
    /// assert_eq!(result, "this");
    /// assert_eq!(reader.byte_offset(), 0);
    ///
    /// let result = reader.peek_while(|i,c| i < 3);
    /// assert_eq!(result, "thi");
    /// assert_eq!(reader.byte_offset(), 0);
    ///
    /// let result = reader.peek_while(|i,c| ('0'..='9').contains(&c));
    /// assert_eq!(result, "");
    /// assert_eq!(reader.byte_offset(), 0);
    /// ```
    pub fn peek_while<F>(&mut self, mut condition: F) -> &'a str
    where
        F: FnMut(usize, char) -> bool,
    {
        let remaining = self.remaining_content();

        let mut offset = 0;
        for (i, char) in remaining.chars().enumerate() {
            if !condition(i, char) {
                break;
            }

            offset += char.len_utf8();
        }

        &remaining[0..offset]
    }

    /// Gets a `Span` that contains the susbstring delimited by both (`from`, `to`) cursors.
    /// The order of the cursors does not matter.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("this test");
    ///
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('h'));
    /// let from = reader.save_cursor();
    ///
    /// assert_eq!(reader.read(), Some('i'));
    /// assert_eq!(reader.read(), Some('s'));
    /// assert_eq!(reader.read(), Some(' '));
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('e'));
    /// assert_eq!(reader.read(), Some('s'));
    /// let to = reader.save_cursor();
    ///
    /// assert_eq!(reader.substring(&from, &to).content(), "is tes");
    /// assert_eq!(reader.substring(&to, &from).content(), "is tes");
    /// ```
    pub fn substring(&self, from: &Cursor, to: &Cursor) -> Span {
        let (from, to) = if from.byte_offset() <= to.byte_offset() {
            (from, to)
        } else {
            (to, from)
        };

        Span::new(self.content, from.clone(), to.clone())
    }

    /// Gets a `Span` that contains the susbstring delimited by `cursor` and current cursors.
    /// The order of the cursors does not matter.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("this test");
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('h'));
    /// let from = reader.save_cursor();
    ///
    /// assert_eq!(reader.read(), Some('i'));
    /// assert_eq!(reader.read(), Some('s'));
    /// assert_eq!(reader.read(), Some(' '));
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('e'));
    /// assert_eq!(reader.read(), Some('s'));
    ///
    /// assert_eq!(reader.substring_to_current(&from).content(), "is tes");
    /// ```
    pub fn substring_to_current(&self, cursor: &Cursor) -> Span {
        self.substring(&self.cursor, cursor)
    }

    /// Saves the current `Reader`'s position as a new `Cursor`.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("this test");
    ///
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('h'));
    /// let cursor = reader.save_cursor();
    ///
    /// assert_eq!(cursor.byte_offset(), 2);
    /// ```
    pub fn save_cursor(&self) -> Cursor {
        self.cursor.clone()
    }

    /// Restores the reader to the specified `Cursor` state.
    ///
    /// # Safety
    ///
    /// This method will cause undefined behaviour if the Cursor was not generated by this reader.
    ///
    /// # Example
    ///
    /// ```
    /// # use parfet::Reader;
    /// let mut reader = Reader::new("this test");
    /// let cursor = reader.save_cursor();
    ///
    /// assert_eq!(reader.byte_offset(), 0);
    /// assert_eq!(cursor.byte_offset(), 0);
    ///
    /// assert_eq!(reader.read(), Some('t'));
    /// assert_eq!(reader.read(), Some('h'));
    /// let cursor2 = reader.save_cursor();
    ///
    /// assert_eq!(reader.byte_offset(), 2);
    /// assert_eq!(cursor.byte_offset(), 0);
    /// assert_eq!(cursor2.byte_offset(), 2);
    ///
    /// reader.restore(cursor);
    ///
    /// assert_eq!(reader.byte_offset(), 0);
    /// assert_eq!(cursor2.byte_offset(), 2);
    /// ```
    pub fn restore(&mut self, cursor: Cursor) {
        self.cursor = cursor;
    }

    /// Consumes `count` bytes moving the start index forward.
    fn consume(&mut self, count: usize) {
        assert!(
            self.remaining_length() >= count,
            "count is greater than end position",
        );

        // Speed up method.
        if count == 0 {
            return;
        }

        let offset = self.byte_offset();
        let new_offset = offset + count;
        let consumed_fragment = &self.content[offset..new_offset];
        let additional_chars = num_chars(consumed_fragment.as_bytes());
        let additional_lines = Memchr::new(b'\n', consumed_fragment.as_bytes()).count();

        // When the line change, count previous characters. Otherwise count only consumed chars to speed-up.
        let new_column = if additional_lines == 0 {
            self.column() + num_chars(consumed_fragment.as_bytes())
        } else {
            let bytes_before_self = &self.content[..new_offset];
            let start_position = match memchr::memrchr(b'\n', bytes_before_self.as_bytes()) {
                Some(pos) => new_offset - pos,
                None => new_offset + 1,
            };

            num_chars(bytes_before_self[new_offset - (start_position - 1)..].as_bytes()) + 1
        };

        self.cursor = Cursor::new(
            new_offset,
            self.char_offset() + additional_chars,
            self.line() + additional_lines,
            new_column,
        );
    }

    // STATIC METHODS ---------------------------------------------------------

    /// Checks whether `char` is contained in `interval`.
    fn check_inside(char: char, interval: &[RangeInclusive<char>]) -> bool {
        for range in interval {
            // Exit early to optimize searching.
            if &char < range.start() {
                break;
            }

            if range.contains(&char) {
                return true;
            }
        }

        false
    }
}

impl<'a, C: Clone> Reader<'a, C> {
    // GETTERS ----------------------------------------------------------------

    /// The remaining content as an `Span`.
    pub fn remaining_content_span(&self) -> Span<'a> {
        let mut aux_reader = self.clone();
        aux_reader.consume(self.remaining_length());

        Span::new(self.content, self.cursor.clone(), aux_reader.cursor)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume_0() {
        let text = "This\nis\nthe\nfragment";
        let mut reader = Reader::new(text);
        reader.consume(0);

        assert_eq!(reader.byte_offset(), 0, "The offset is incorrect");
        assert_eq!(reader.char_offset(), 0, "The char_offset is incorrect");
        assert_eq!(reader.line(), 1, "The line is incorrect");
        assert_eq!(reader.column(), 1, "The column is incorrect");
    }

    #[test]
    fn test_consume() {
        let text = "This\nis\nthe\nfragment";
        let mut reader = Reader::new(text);
        reader.consume(2);

        assert_eq!(reader.byte_offset(), 2, "The offset is incorrect");
        assert_eq!(reader.char_offset(), 2, "The char_offset is incorrect");
        assert_eq!(reader.line(), 1, "The line is incorrect");
        assert_eq!(reader.column(), 3, "The column is incorrect");

        reader.consume(3);

        assert_eq!(reader.byte_offset(), 5, "The offset is incorrect");
        assert_eq!(reader.char_offset(), 5, "The char_offset is incorrect");
        assert_eq!(reader.line(), 2, "The line is incorrect");
        assert_eq!(reader.column(), 1, "The column is incorrect");

        reader.consume(2);

        assert_eq!(reader.byte_offset(), 7, "The offset is incorrect");
        assert_eq!(reader.char_offset(), 7, "The char_offset is incorrect");
        assert_eq!(reader.line(), 2, "The line is incorrect");
        assert_eq!(reader.column(), 3, "The column is incorrect");
    }

    #[test]
    fn test_consume_utf_chars() {
        let text = "モスフェト";
        let mut reader = Reader::new(text);
        reader.consume(3);

        assert_eq!(reader.byte_offset(), 3, "The offset is incorrect");
        assert_eq!(reader.char_offset(), 1, "The char_offset is incorrect");
        assert_eq!(reader.line(), 1, "The line is incorrect");
        assert_eq!(reader.column(), 2, "The column is incorrect");
    }
}
