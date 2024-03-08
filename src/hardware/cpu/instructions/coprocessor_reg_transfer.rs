use crate::hardware::{cpu::CPU, memory::Memory};

pub fn mcr<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn mcrr<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn mrc<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn mrrc<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
