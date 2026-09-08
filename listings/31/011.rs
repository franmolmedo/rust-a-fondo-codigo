struct DeviceBuffer {
    data: *mut u8, // Raw pointers prevent automatic Send and Sync.
    len: usize,
}

impl DeviceBuffer {
    fn write(&mut self, offset: usize, byte: u8) {
        assert!(offset < self.len);
        // SAFETY: the type owns a valid, writable region of len bytes.
        // The checked offset is in bounds and &mut self ensures exclusive access.
        unsafe { self.data.add(offset).write(byte) }
    }
}

// SAFETY: constructors must establish exclusive ownership of the allocation;
// mutation requires &mut self and Drop must free it exactly once.
// Access and deallocation must be valid from any thread that owns the value.
unsafe impl Send for DeviceBuffer {}
