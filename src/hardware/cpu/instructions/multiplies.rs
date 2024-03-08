use crate::hardware::{cpu::CPU, memory::Memory};

pub fn mla<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn mul<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smlal<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smull<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn umlal<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn umull<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
