use std::borrow::Borrow;
use std::io::{Result, Write};

/// This prints the byte string, stylizing it based on its internal state.
/// Its main method is `Printer::into_string`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Printer {
    quotes: Quotes,
}

/// Indicates how, if at all, to quote the byte string.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Quotes {
    None,
    Single,
    Double,
}

impl Default for Quotes {
    fn default() -> Self {
        Self::None
    }
}

/// A helper function to print bytes without explicitly creating a Printer struct.
/// Equivalent to `println!("{}", Printer::default().into_string(bytes))`
pub fn println<I>(bytes: I)
where
    I: IntoIterator,
    I::Item: std::borrow::Borrow<u8>,
{
    println!("{}", Printer::default().into_string(bytes));
}

impl Default for Printer {
    fn default() -> Self {
        Self {
            quotes: Quotes::None,
        }
    }
}

impl Printer {
    /// Returns a new Printer, equivalent to `Printer::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new Printer wrapping its output in single quotes. It escapes any single quotes within the output.
    pub fn with_single_quotes() -> Self {
        Self {
            quotes: Quotes::Single,
        }
    }

    /// Returns a new Printer wrapping its output in double quotes. It escapes any double quotes within the output.
    pub fn with_double_quotes() -> Self {
        Self {
            quotes: Quotes::Double,
        }
    }

    /// Writes the output to an arbitrary writer.
    pub fn write_to<I, W>(&self, writer: &mut W, bytes: I) -> Result<()>
    where
        I: IntoIterator,
        I::Item: Borrow<u8>,
        W: Write,
    {
        writer.write_all(self.into_string(bytes).as_bytes())
    }

    /// Returns a string displaying the bytes.
    pub fn into_string<I>(&self, bytes: I) -> String
    where
        I: IntoIterator,
        I::Item: Borrow<u8>,
    {
        let mut output = String::new();

        match self.quotes {
            Quotes::None => (),
            Quotes::Single => output.push('\''),
            Quotes::Double => output.push('"'),
        }

        for byte_borrow in bytes.into_iter() {
            let byte = *byte_borrow.borrow();
            if byte.is_ascii_graphic() {
                match self.quotes {
                    Quotes::Single if byte == b'\'' => output.push('\\'),
                    Quotes::Double if byte == b'"' => output.push('\\'),
                    _ if byte == b'\\' => output.push('\\'),
                    _ => (),
                }
                output.push(byte as char);
            } else {
                output.push_str(&format!("\\x{:02x}", byte));
            }
        }

        match self.quotes {
            Quotes::None => (),
            Quotes::Single => output.push('\''),
            Quotes::Double => output.push('"'),
        }

        output
    }
}
