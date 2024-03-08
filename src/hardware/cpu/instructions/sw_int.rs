use crate::hardware::{cpu::CPU, memory::Memory};

pub fn swi<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
