use course_solutions::unsafe_low_level::c46::{DecodeError, Percentage, RecordHeader, RecordTag};

#[test]
fn c46_every_input_byte_is_validated_before_building_domain_values() {
    for value in 0_u8..=u8::MAX {
        assert_eq!(Percentage::new(value).is_ok(), value <= 100);
        assert_eq!(RecordTag::try_from(value).is_ok(), matches!(value, 1 | 2));
    }
}

#[test]
fn c46_header_decoding_has_an_explicit_prefix_only_contract() {
    let header = RecordHeader {
        version: u16::MAX,
        tag: RecordTag::User,
        payload_length: u32::MAX,
    };
    let bytes = header.encode();
    for length in 0..RecordHeader::ENCODED_LENGTH {
        assert_eq!(
            RecordHeader::decode(&bytes[..length]),
            Err(DecodeError::Truncated)
        );
    }
    assert_eq!(RecordHeader::decode(&bytes), Ok(header));
    let mut message = bytes.to_vec();
    message.extend_from_slice(b"payload");
    assert_eq!(RecordHeader::decode(&message), Ok(header));
}
