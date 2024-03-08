use crate::hardware::{cpu::CPU, memory::Memory};

pub fn swi<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
