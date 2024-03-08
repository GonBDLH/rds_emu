use crate::hardware::{cpu::CPU, memory::Memory};

pub fn cdp<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
