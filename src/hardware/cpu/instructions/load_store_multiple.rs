use crate::hardware::{cpu::CPU, memory::Memory};

pub fn ldm_1<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldm_2<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn ldm_3<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn stm<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn stm_2<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
