use crate::hardware::{cpu::CPU, memory::Memory};

pub fn bkpt<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn blx_miscelaneous<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn bx<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn bxj<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn clz<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn mrs<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn qadd<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn qdadd<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn qdsub<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn qsub<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smlaxy<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smlalxy<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smlawy<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smulxy<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}

pub fn smulwy<T: CPU>(_cpu: &mut T, _mem: &mut Memory) {}
