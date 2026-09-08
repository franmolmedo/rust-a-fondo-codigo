//! One request per connection: SUM <decimal-u64> <decimal-u64> followed by LF.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reply {
    Value(u64),
    InvalidRequest,
    Overflow,
}

impl Reply {
    pub fn to_line(self) -> String {
        match self {
            Self::Value(value) => format!("OK {value}\n"),
            Self::InvalidRequest => "ERR invalid request\n".into(),
            Self::Overflow => "ERR overflow\n".into(),
        }
    }
}

pub fn evaluate(request: &[u8]) -> Reply {
    let Some(line) = request.strip_suffix(b"\n") else {
        return Reply::InvalidRequest;
    };
    let line = line.strip_suffix(b"\r").unwrap_or(line);
    let Ok(text) = std::str::from_utf8(line) else {
        return Reply::InvalidRequest;
    };
    let mut fields = text.split(' ');
    let (Some("SUM"), Some(left), Some(right), None) =
        (fields.next(), fields.next(), fields.next(), fields.next())
    else {
        return Reply::InvalidRequest;
    };
    let decimal = |field: &str| -> Option<u64> {
        if field.is_empty() || !field.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        field.parse().ok()
    };
    let (Some(left), Some(right)) = (decimal(left), decimal(right)) else {
        return Reply::InvalidRequest;
    };
    left.checked_add(right)
        .map_or(Reply::Overflow, Reply::Value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_lf_crlf_and_leading_zeroes() {
        assert_eq!(evaluate(b"SUM 19 23\n"), Reply::Value(42));
        assert_eq!(evaluate(b"SUM 00 01\r\n"), Reply::Value(1));
    }

    #[test]
    fn rejects_malformed_frames() {
        for request in [
            b"SUM 1 2".as_slice(),
            b"SUM  1 2\n",
            b"SUM +1 2\n",
            b"SUM -1 2\n",
            b"SUM 1 2 3\n",
            b"sum 1 2\n",
            b"SUM 1 \xff\n",
            b"SUM 1\n2\n",
            b"SUM 1 18446744073709551616\n",
            b"\n",
        ] {
            assert_eq!(evaluate(request), Reply::InvalidRequest);
        }
    }

    #[test]
    fn overflow_is_a_protocol_error() {
        assert_eq!(evaluate(b"SUM 18446744073709551615 1\n"), Reply::Overflow);
        assert_eq!(Reply::Overflow.to_line(), "ERR overflow\n");
        assert_eq!(Reply::Value(42).to_line(), "OK 42\n");
    }
}
