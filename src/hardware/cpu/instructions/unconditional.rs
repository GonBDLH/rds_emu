use crate::hardware::{cpu::CPU, memory::Memory};

pub fn blx_unconditional<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn cps<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn pld<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
