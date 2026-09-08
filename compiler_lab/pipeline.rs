//! Minimal source compiled directly with `rustc` by the chapter 52 lab. It
//! exposes MIR, LLVM IR, and assembly without depending on Cargo.

use std::ops::Add;

pub struct Ticket {
    label: String,
}

impl Ticket {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

#[inline(never)]
pub fn inspect_ticket(ticket: Ticket) -> usize {
    ticket.label.len()
}

#[inline(never)]
pub fn twice<T>(value: T) -> T
where
    T: Copy + Add<Output = T>,
{
    value + value
}

// SAFETY: this lab owns the uniquely prefixed symbol with this C ABI.
#[unsafe(no_mangle)]
pub extern "C" fn c52_exported_add(left: u32, right: u32) -> u32 {
    left.wrapping_add(right)
}

// SAFETY: this lab owns the uniquely prefixed symbol with this C ABI.
#[unsafe(no_mangle)]
pub extern "C" fn c52_twice_u64(value: u64) -> u64 {
    twice(std::num::Wrapping(value)).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_contract_survives_the_observation_build() {
        assert_eq!(inspect_ticket(Ticket::new("MIR")), 3);
        assert_eq!(c52_exported_add(20, 22), 42);
        assert_eq!(c52_twice_u64(21), 42);
        assert_eq!(c52_exported_add(u32::MAX, 1), 0);
        assert_eq!(c52_twice_u64(u64::MAX), u64::MAX - 1);
    }
}
