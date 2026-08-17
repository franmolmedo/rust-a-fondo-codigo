use std::ffi::c_int;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Status {
    Ready,
    Busy,
    Unknown(c_int),
}

fn decode_status(code: c_int) -> Status {
    match code {
        0 => Status::Ready,
        1 => Status::Busy,
        other => Status::Unknown(other),
    }
}

assert_eq!(decode_status(91), Status::Unknown(91));
