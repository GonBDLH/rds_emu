use crate::hardware::{cpu::CPU, memory::Memory};

use super::Instruction;

pub fn b_bl<T: CPU>(_instr: &Instruction<T>, _cpu: &mut T, _mem: &mut Memory) {}
