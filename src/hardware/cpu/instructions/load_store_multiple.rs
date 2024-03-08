use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldm_1<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldm_2<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn ldm_3<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn stm<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn stm_2<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
