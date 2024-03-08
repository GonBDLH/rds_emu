use crate::hardware::{cpu::CPU, memory::Memory};

pub fn bkpt<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn blx_miscelaneous<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn bx<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn bxj<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn clz<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn mrs<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn qadd<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn qdadd<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn qdsub<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn qsub<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smlaxy<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smlalxy<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smlawy<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smulxy<T: CPU>(cpu: &mut T, mem: &mut Memory) {}

pub fn smulwy<T: CPU>(cpu: &mut T, mem: &mut Memory) {}
