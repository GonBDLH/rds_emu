use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldr<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldrb<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldrbt<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldrt<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn str_fn<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn strb<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn strbt<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn strt<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
