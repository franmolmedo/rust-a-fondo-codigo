use std::mem::{align_of, size_of};

#[repr(transparent)]
struct FileDescriptor(i32);

fn main() {
    assert_eq!(size_of::<FileDescriptor>(), size_of::<i32>());
    assert_eq!(align_of::<FileDescriptor>(), align_of::<i32>());
}
