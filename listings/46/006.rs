#[repr(C, u8)]
enum Event {
    Started { job_id: u32 },
    Finished { job_id: u32, code: u16 },
    Stopped,
}

let event = Event::Finished {
    job_id: 7,
    code: 0,
};
assert!(std::mem::size_of_val(&event) >= std::mem::size_of::<u32>());
