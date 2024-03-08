use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldc<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn stc<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn stc_2<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
