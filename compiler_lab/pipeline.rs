//! Fuente mínima que el laboratorio del capítulo 52 compila directamente con
//! `rustc` para observar MIR, LLVM IR y assembly sin depender de Cargo.

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

#[unsafe(no_mangle)]
pub extern "C" fn c52_exported_add(left: u32, right: u32) -> u32 {
    left + right
}

#[unsafe(no_mangle)]
pub extern "C" fn c52_twice_u64(value: u64) -> u64 {
    twice(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_contract_survives_the_observation_build() {
        assert_eq!(inspect_ticket(Ticket::new("MIR")), 3);
        assert_eq!(c52_exported_add(20, 22), 42);
        assert_eq!(c52_twice_u64(21), 42);
    }
}
