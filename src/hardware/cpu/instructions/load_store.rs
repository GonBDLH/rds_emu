use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldr<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldrb<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldrbt<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldrt<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn str_fn<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn strb<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn strbt<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn strt<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
