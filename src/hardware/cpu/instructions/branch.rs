use crate::hardware::{cpu::CPU, memory::Memory};

use super::Instruction;

pub fn b_bl<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {

}
