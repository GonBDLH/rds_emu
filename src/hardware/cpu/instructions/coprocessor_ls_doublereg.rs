use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldc<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn stc<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn stc_2<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
