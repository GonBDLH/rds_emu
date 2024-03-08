use crate::hardware::{cpu::CPU, memory::Memory};

pub fn mcr<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn mcrr<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn mrc<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn mrrc<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
