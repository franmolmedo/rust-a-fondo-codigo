#![no_main]

use libfuzzer_sys::fuzz_target;
use rust_appendix_lab::data::parse_record;

fuzz_target!(|bytes: &[u8]| {
    if bytes.len() > 1024 {
        return;
    }
    if let Ok(text) = std::str::from_utf8(bytes)
        && let Ok(record) = parse_record(text)
    {
        let serialized = record.to_line();
        assert_eq!(parse_record(&serialized).unwrap(), record);
    }
});
