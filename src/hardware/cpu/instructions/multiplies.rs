use crate::hardware::{cpu::CPU, memory::Memory};

pub fn mla<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn mul<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smlal<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smull<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn umlal<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn umull<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
