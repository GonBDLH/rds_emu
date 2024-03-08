use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldrd<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldrh<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldrsb<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldrsh<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn msr_reg<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn strd<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn strh<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn swp<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn swpb<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
