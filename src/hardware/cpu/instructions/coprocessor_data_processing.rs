use crate::hardware::{cpu::CPU, memory::Memory};

pub fn cdp<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
