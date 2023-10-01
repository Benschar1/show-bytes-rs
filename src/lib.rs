//! # show-bytes
//!
//! Display bytes as printable ascii with escape sequences as needed.
//!
//! ## Examples
//!
//! ```rust
//! use show_bytes::println;
//!
//! // byte slice
//! let bytes_slice: &[u8] = &[72, 101, 108, 108, 111, 0, 255];
//! println(bytes_slice);
//!
//! // byte vector
//! let bytes_vec: Vec<u8> = vec![72, 101, 108, 108, 111, 0, 255];
//! println(&bytes_vec);
//! println(bytes_vec);
//!
//! // byte array
//! let bytes_array: [u8; 7] = [72, 101, 108, 108, 111, 0, 255];
//! println(bytes_array);
//! println(&bytes_array);
//!
//! // byte iterator
//! let mut bytes_iter = [72, 101, 108, 108, 111, 0, 255].iter();
//! println(bytes_iter.clone());
//! println(&mut bytes_iter);
//!
//! // &str
//! let bytes_str: &str = "hello\0\x7f";
//! println(bytes_str.bytes());
//! let bytes_str = &bytes_str;
//! println(bytes_str.bytes());
//!
//! // String
//! let bytes_string: String = bytes_str.to_string();
//! println(bytes_string.bytes());
//! let bytes_string = &bytes_string;
//! println(bytes_string.bytes());
//!
//! // OsString
//! let bytes_os_string: OsString = OsString::from(bytes_str);
//! println(bytes_os_string.as_bytes());
//! let bytes_os_string: &OsString = &bytes_os_string;
//! println(bytes_os_string.as_bytes());
//!
//! // OsStr
//! let bytes_os_str: &OsStr = OsStr::from_bytes(bytes_slice);
//! println(bytes_os_str.as_bytes());
//!
//! // Box<[u8]>
//! let boxed_slice: Box<[u8]> = Box::new([72, 101, 108, 108, 111, 0, 255]);
//! println(boxed_slice.iter());
//! println(&mut boxed_slice.iter());
//!
//! // std::io::Cursor<Vec<u8>>
//! let cursor = Cursor::new(vec![72, 101, 108, 108, 111, 0, 255]);
//! let bytes_from_cursor: Vec<u8> = cursor.into_inner();
//! println(&bytes_from_cursor);
//! println(bytes_from_cursor);
//!
//! // std::collections::VecDeque<u8>
//! let mut vec_deque = VecDeque::new();
//! vec_deque.push_back(72);
//! vec_deque.push_back(101);
//! vec_deque.push_back(108);
//! vec_deque.push_back(108);
//! vec_deque.push_back(111);
//! vec_deque.push_back(0);
//! vec_deque.push_back(255);
//! println(&vec_deque);
//! println(vec_deque);
//!
//! // Cow<[u8]>
//! let cow_slice: Cow<[u8]> = Cow::Borrowed(&[72, 101, 108, 108, 111, 0, 255]);
//! println(cow_slice.iter());
//! let cow_slice: Cow<[u8]> = Cow::Owned(vec![72, 101, 108, 108, 111, 0, 255]);
//! println(cow_slice.iter());
//!
//! // Arc<Vec<u8>>
//! let arc_slice = Arc::new(vec![72, 101, 108, 108, 111, 0, 255]);
//! println(arc_slice.iter());
//!
//! // Rc<Vec<u8>>
//! let rc_slice = Rc::new(vec![72, 101, 108, 108, 111, 0, 255]);
//! println(rc_slice.iter());
//! ```

use std::borrow::Borrow;
use std::fmt::{Result, Write};

/// A helper function to show bytes without explicitly creating a Printer struct.
///
/// `show_bytes(bytes)` is equivalent to `Printer::new(QuoteStyle::Double).into_string(bytes)`.
pub fn show_bytes<I>(bytes: I) -> String
where
    I: IntoIterator,
    I::Item: Borrow<u8>,
{
    Printer::new(QuoteStyle::Double).into_string(bytes)
}

/// Maintains an internal state describing how to display a byte array.
///
/// It can write to an arbitrary `std::fmt::Write` implementation using the
/// method `Printer::write_to`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Printer {
    quote_style: QuoteStyle,
}

/// Indicates how, if at all, the printer should quote the byte string.
///
/// A printer's quoting style is chosen by passing a `QuoteStyle` to `Printer::new`.
/// The choice of quoting style affects which characters are escaped.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum QuoteStyle {
    /// Indicates that the printer shouldn't quote its output.
    None,
    /// Indicates that the printer should wrap its output in single quotes, and
    /// escape any single quotes within the output.
    Single,
    /// Indicates that the printer should wrap its output in double quotes, and
    /// escape anydouble quotes within the output.
    Double,
}

/// Returns `Quotes::None`.
impl Default for QuoteStyle {
    fn default() -> Self {
        Self::None
    }
}

/// Returns a printer that doesn't quote its output.
///
/// This is equivalent to `Printer::new(QuoteStyle::None)`.
impl Default for Printer {
    fn default() -> Self {
        Self {
            quote_style: QuoteStyle::None,
        }
    }
}

impl Printer {
    /// Returns a new Printer with the chosen quoting style.
    pub fn new(quote_style: QuoteStyle) -> Self {
        Self { quote_style }
    }

    /// Writes the formatted bytes to an arbitrary `std::fmt::Write` implementation.
    ///
    /// This method iterates through the byte array and calls `writer.write_char`
    /// or `writer.write_str` as appropriate. The performance of this method thus
    /// depends on the implementation of the underlying writer. If a writer doesn't
    /// implement good buffering, or if each call to `write` makes a system call
    /// to do IO, this can be inefficient.
    ///
    /// The helper method `Printer::into_string` is a thin wrapper around this
    /// method, calling `write_to` with a `String` as its writer. `String` implements
    /// `fmt::Write` with methods like `String::push` and `String::push_str`, which
    /// should be adequately performant for most uses. If the use case calls for it,
    /// the user can always implement their own writer with custom buffering.
    pub fn write_to<I, W>(&self, bytes: I, writer: &mut W) -> Result
    where
        I: IntoIterator,
        I::Item: Borrow<u8>,
        W: Write,
    {
        match self.quote_style {
            QuoteStyle::None => Ok(()),
            QuoteStyle::Single => writer.write_char('\''),
            QuoteStyle::Double => writer.write_char('"'),
        }?;

        for byte_borrow in bytes.into_iter() {
            let byte = *byte_borrow.borrow();
            match self.quote_style {
                QuoteStyle::Single if byte == b'\'' => writer.write_str("\\'"),
                QuoteStyle::Double if byte == b'"' => writer.write_str("\\\""),
                _ if byte == b'\\' => writer.write_str("\\\\"),
                _ if byte.is_ascii_graphic() => writer.write_char(byte as char),
                _ => write!(writer, "\\x{:02x}", byte),
            }?;
        }

        match self.quote_style {
            QuoteStyle::None => Ok(()),
            QuoteStyle::Single => writer.write_char('\''),
            QuoteStyle::Double => writer.write_char('"'),
        }?;

        Ok(())
    }

    /// Returns a string displaying the bytes.
    pub fn into_string<I>(&self, bytes: I) -> String
    where
        I: IntoIterator,
        I::Item: Borrow<u8>,
    {
        let mut output = String::new();

        self.write_to(bytes, &mut output).expect(
            "Writing to a string shouldn't fail, it uses infallible methods like String::push.",
        );

        output
    }
}
