use crate::hardware::{cpu::CPU, memory::Memory};

pub fn msr_imm<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
