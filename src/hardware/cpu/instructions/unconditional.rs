use crate::hardware::{cpu::CPU, memory::Memory};

pub fn blx_unconditional<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn cps<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn pld<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
