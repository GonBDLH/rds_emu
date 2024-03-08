use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldrd<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldrh<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldrsb<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldrsh<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn msr_reg<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn strd<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn strh<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn swp<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn swpb<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
