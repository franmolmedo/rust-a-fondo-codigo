//! Bounded, UTF-8 inventory input shared by appendices B, D, E and F.

use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, BufRead, Write};
use std::num::ParseIntError;
use std::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    name: String,
    quantity: u64,
}

impl Record {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn quantity(&self) -> u64 {
        self.quantity
    }

    pub fn to_line(&self) -> String {
        format!("{},{}\n", self.name, self.quantity)
    }
}

#[derive(Debug)]
pub enum RecordError {
    MissingSeparator,
    ExtraField,
    InvalidName,
    InvalidQuantity(ParseIntError),
}

impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSeparator => f.write_str("expected name,quantity"),
            Self::ExtraField => f.write_str("expected exactly two fields"),
            Self::InvalidName => {
                f.write_str("name must contain 1..=64 bytes and no control characters")
            }
            Self::InvalidQuantity(_) => f.write_str("quantity must fit in u64"),
        }
    }
}

impl std::error::Error for RecordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidQuantity(error) => Some(error),
            _ => None,
        }
    }
}

/// Parses a single record without a line terminator. This is not RFC 4180 CSV.
pub fn parse_record(line: &str) -> Result<Record, RecordError> {
    let (name, quantity) = line.split_once(',').ok_or(RecordError::MissingSeparator)?;
    if quantity.contains(',') {
        return Err(RecordError::ExtraField);
    }
    let name = name.trim();
    if name.is_empty() || name.len() > 64 || name.chars().any(char::is_control) {
        return Err(RecordError::InvalidName);
    }
    let quantity = quantity
        .trim()
        .parse()
        .map_err(RecordError::InvalidQuantity)?;
    Ok(Record {
        name: name.to_owned(),
        quantity,
    })
}

/// Reads at most `max_bytes`, including a possible LF delimiter.
/// An overlong line is an error; this function does not skip its remaining bytes.
pub fn read_bounded_line<R: BufRead>(
    reader: &mut R,
    output: &mut Vec<u8>,
    max_bytes: usize,
) -> io::Result<usize> {
    if max_bytes == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "zero line limit",
        ));
    }
    output.clear();
    loop {
        let available = match reader.fill_buf() {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if available.is_empty() {
            return Ok(output.len());
        }
        let newline = available.iter().position(|&byte| byte == b'\n');
        let count = newline.map_or(available.len(), |index| index + 1);
        if count > max_bytes.saturating_sub(output.len()) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "line byte limit exceeded",
            ));
        }
        output.extend_from_slice(&available[..count]);
        reader.consume(count);
        if newline.is_some() {
            return Ok(output.len());
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub line_bytes: usize,
    pub total_bytes: u64,
    pub distinct_items: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            line_bytes: 256,
            total_bytes: 1_048_576,
            distinct_items: 1024,
        }
    }
}

#[derive(Debug)]
pub enum ImportError {
    InvalidLimits,
    Io { line: usize, source: io::Error },
    Utf8 { line: usize, source: Utf8Error },
    Record { line: usize, source: RecordError },
    TotalBytes,
    TooManyItems,
    QuantityOverflow { line: usize, name: String },
}

impl fmt::Display for ImportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLimits => f.write_str("limits must be positive and representable"),
            Self::Io { line, source } => write!(f, "line {line}: input failed: {source}"),
            Self::Utf8 { line, .. } => write!(f, "line {line}: invalid UTF-8"),
            Self::Record { line, source } => write!(f, "line {line}: {source}"),
            Self::TotalBytes => f.write_str("input byte limit exceeded"),
            Self::TooManyItems => f.write_str("distinct item limit exceeded"),
            Self::QuantityOverflow { line, name } => {
                write!(f, "line {line}: total for {name} exceeds u64")
            }
        }
    }
}

impl std::error::Error for ImportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Utf8 { source, .. } => Some(source),
            Self::Record { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Builds a report only if the entire input is valid. No external writes occur.
pub fn summarize<R: BufRead>(
    mut reader: R,
    limits: Limits,
) -> Result<BTreeMap<String, u64>, ImportError> {
    if limits.line_bytes == 0
        || limits.total_bytes == 0
        || limits.distinct_items == 0
        || u64::try_from(limits.line_bytes).is_err()
    {
        return Err(ImportError::InvalidLimits);
    }
    let mut totals = BTreeMap::new();
    let mut buffer = Vec::new();
    let mut bytes = 0_u64;
    let mut line = 0_usize;
    loop {
        line = line.checked_add(1).ok_or(ImportError::TotalBytes)?;
        let count = read_bounded_line(&mut reader, &mut buffer, limits.line_bytes)
            .map_err(|source| ImportError::Io { line, source })?;
        if count == 0 {
            break;
        }
        bytes = bytes
            .checked_add(count as u64)
            .ok_or(ImportError::TotalBytes)?;
        if bytes > limits.total_bytes {
            return Err(ImportError::TotalBytes);
        }
        let text =
            std::str::from_utf8(&buffer).map_err(|source| ImportError::Utf8 { line, source })?;
        let text = text.strip_suffix('\n').unwrap_or(text);
        let text = text.strip_suffix('\r').unwrap_or(text);
        let record = parse_record(text).map_err(|source| ImportError::Record { line, source })?;
        let previous: u64 = totals.get(record.name()).copied().unwrap_or(0);
        let updated = previous.checked_add(record.quantity()).ok_or_else(|| {
            ImportError::QuantityOverflow {
                line,
                name: record.name().to_owned(),
            }
        })?;
        if !totals.contains_key(record.name()) && totals.len() >= limits.distinct_items {
            return Err(ImportError::TooManyItems);
        }
        totals.insert(record.name, updated);
    }
    Ok(totals)
}

/// Writes sorted UTF-8 rows. Names must come from the validated importer.
pub fn write_report<W: Write>(mut writer: W, totals: &BTreeMap<String, u64>) -> io::Result<()> {
    for (name, quantity) in totals {
        writeln!(writer, "{name},{quantity}")?;
    }
    writer.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};

    #[test]
    fn sorted_totals_accept_unicode_crlf_and_unterminated_last_line() {
        let input = "washer,2\r\ntuerca,3\nwasher,4\ncafé,1";
        let totals = summarize(Cursor::new(input.as_bytes()), Limits::default()).unwrap();
        let mut output = Vec::new();
        write_report(&mut output, &totals).unwrap();
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "café,1\ntuerca,3\nwasher,6\n"
        );
    }

    #[test]
    fn empty_input_is_an_empty_report() {
        assert!(
            summarize(Cursor::new(b""), Limits::default())
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn invalid_records_are_rejected() {
        for input in ["", "x", "x,1,2", ",1", "x,-1", "x,1.5", "x,", "a\tb,1"] {
            assert!(parse_record(input).is_err(), "{input:?}");
        }
        assert!(parse_record(&format!("{},1", "x".repeat(65))).is_err());
        assert_eq!(parse_record("x,0").unwrap().quantity(), 0);
    }

    #[test]
    fn invalid_utf8_is_not_replaced_silently() {
        assert!(matches!(
            summarize(Cursor::new(b"\xff,1\n"), Limits::default()),
            Err(ImportError::Utf8 { line: 1, .. })
        ));
    }

    #[test]
    fn preserves_parse_error_source() {
        use std::error::Error;
        let error = summarize(Cursor::new(b"x,not-a-number"), Limits::default()).unwrap_err();
        assert!(error.source().unwrap().source().is_some());
    }

    #[test]
    fn accumulation_never_wraps() {
        let input = format!("x,{}\nx,1\n", u64::MAX);
        assert!(matches!(
            summarize(Cursor::new(input), Limits::default()),
            Err(ImportError::QuantityOverflow { line: 2, .. })
        ));
    }

    #[test]
    fn bounds_lines_before_allocating_the_complete_input() {
        let mut reader = BufReader::with_capacity(2, Cursor::new(b"abcdef\n"));
        let mut output = Vec::new();
        assert!(read_bounded_line(&mut reader, &mut output, 4).is_err());
        assert!(output.len() <= 4);
    }

    #[test]
    fn exact_line_limit_includes_newline() {
        let mut output = Vec::new();
        assert_eq!(
            read_bounded_line(&mut Cursor::new(b"x,1\n"), &mut output, 4).unwrap(),
            4
        );
        assert!(read_bounded_line(&mut Cursor::new(b"x,1\n"), &mut output, 3).is_err());
    }

    #[test]
    fn total_bytes_and_distinct_items_are_independent_limits() {
        let small = Limits {
            total_bytes: 3,
            ..Limits::default()
        };
        assert!(matches!(
            summarize(Cursor::new(b"x,1\n"), small),
            Err(ImportError::TotalBytes)
        ));
        let one = Limits {
            distinct_items: 1,
            ..Limits::default()
        };
        assert!(summarize(Cursor::new(b"x,1\nx,2\n"), one).is_ok());
        assert!(matches!(
            summarize(Cursor::new(b"x,1\ny,2\n"), one),
            Err(ImportError::TooManyItems)
        ));
    }

    #[test]
    fn zero_limits_are_rejected() {
        let limits = Limits {
            line_bytes: 0,
            ..Limits::default()
        };
        assert!(matches!(
            summarize(Cursor::new(b""), limits),
            Err(ImportError::InvalidLimits)
        ));
    }

    #[test]
    fn short_writes_are_completed() {
        struct ShortWriter(Vec<u8>);
        impl Write for ShortWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                let count = bytes.len().min(1);
                self.0.extend_from_slice(&bytes[..count]);
                Ok(count)
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut writer = ShortWriter(Vec::new());
        let report = BTreeMap::from([(String::from("x"), 12)]);
        write_report(&mut writer, &report).unwrap();
        assert_eq!(writer.0, b"x,12\n");
    }

    #[test]
    fn explicit_flush_reports_failure() {
        struct FailedFlush;
        impl Write for FailedFlush {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                Ok(bytes.len())
            }
            fn flush(&mut self) -> io::Result<()> {
                Err(io::Error::other("flush failed"))
            }
        }
        assert!(write_report(FailedFlush, &BTreeMap::new()).is_err());
    }
}
