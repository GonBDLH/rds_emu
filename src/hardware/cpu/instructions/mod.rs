use crate::hardware::memory::Memory;

use self::data_processing_shift::shifts::*;

use super::CPU;

use bitmatch::bitmatch;

pub mod branch;
pub mod coprocessor_data_processing;
pub mod coprocessor_ls_doublereg;
pub mod coprocessor_reg_transfer;
pub mod data_processing;
pub mod data_processing_shift;
pub mod extra_load_store;
pub mod load_store;
pub mod load_store_multiple;
pub mod miscelaneous;
pub mod multiplies;
pub mod sw_int;
pub mod unconditional;

pub fn nop<T: CPU>(_instr: &Instruction<T>, _cpu: &mut T, _mem: &mut Memory) {}

struct ShiftOp<T: CPU> {
    a: u32,
    b: u32,
    shift_fn: Option<fn(&T, u32, u32) -> (u32, bool)>,
}

impl<T: CPU> ShiftOp<T> {
    pub fn execute(&self, cpu: &T) -> u32 {
        let (shifter_operand, shifter_carry_out) = (self.shift_fn.unwrap())(cpu, self.a, self.b);
        shifter_operand
    } 
}

impl<T: CPU> Default for ShiftOp<T> {
    fn default() -> Self {
        Self {
            a: 0,
            b: 0,
            shift_fn: None,
        }
    }
}

#[bitmatch]
fn get_shift_op<T: CPU>(sr: u16, i: bool) -> ShiftOp<T> {
    let mut shift = ShiftOp::default();

    if i {
        shift.shift_fn = Some(immediate);
    } else {
        #[bitmatch]
        match sr {
            // Register
            "00000000aaaa" => {
                shift.a = a as u32;
                shift.shift_fn = Some(register);
            }
            // Logical Shift Left by Immediate
            "aaaaa000bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(lsl_imm);

            }
            // Logical Shift Left by Register
            "aaa0001bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(lsl_reg);
            }
            // Logical Shift Right by Immediate
            "aaaaa010bbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(lsr_imm);
            }
            // Logical Shift Right by Register
            "aaa0001bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(lsr_reg);
            },
            // Arithmetic Shift Right by Immediate
            "aaaaa100bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(asr_imm);
            },
            // Arithmetic Shift Right by Register
            "aaaa0101bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(asr_reg);
            },
            // Rotate Right by Immediate
            "aaaaa110bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(ror_imm);
            },
            // Rotate Right by Register
            "aaaa0111bbbb" => {
                shift.a = a as u32;
                shift.b = b as u32;
                shift.shift_fn = Some(ror_reg);
            },
            _ => unreachable!(),
        }
    }

    shift
}

struct DataProcessingArgs<T: CPU> {
    i: bool,
    s: bool,
    rn: u32,
    rd: u32,
    shift_op: ShiftOp<T>
}

enum InstructionArgs<T: CPU> {
    DataProcessing(DataProcessingArgs<T>),
    Branch(bool, u32)
}

pub struct Instruction<T: CPU> {
    f: fn(&Instruction<T>, &mut T, &mut Memory),
    condition: InstructionCondition,
    args: Option<InstructionArgs<T>>,
}

impl<T: CPU> Instruction<T> {
    pub fn execute(&self, cpu: &mut T, mem: &mut Memory) {
        (self.f)(self, cpu, mem);
    }
}

impl<T: CPU> Default for Instruction<T> {
    fn default() -> Self {
        Self {
            f: nop,
            condition: InstructionCondition::default(),
            args: None,
        }
    }
}

#[repr(u32)]
#[derive(Default)]
pub enum InstructionCondition {
    #[default]
    EQ = 0,
    NE,
    CsHs,
    CcLo,
    MI,
    PL,
    VS,
    VC,
    HI,
    LS,
    GE,
    LT,
    GT,
    LE,
    AL,
    NV,
}

impl From<u32> for InstructionCondition {
    fn from(value: u32) -> Self {
        let val = (value & 0xF0000000) >> 27;

        match val {
            0b0000 => Self::EQ,
            0b0001 => Self::NE,
            0b0010 => Self::CsHs,
            0b0011 => Self::CcLo,
            0b0100 => Self::MI,
            0b0101 => Self::PL,
            0b0110 => Self::VS,
            0b0111 => Self::VC,
            0b1000 => Self::HI,
            0b1001 => Self::LS,
            0b1010 => Self::GE,
            0b1011 => Self::LT,
            0b1100 => Self::GT,
            0b1101 => Self::LE,
            0b1110 => Self::AL,
            0b1111 => Self::NV,
            _ => unreachable!(),
        }
    }
}

pub struct InstructionBuilder<T: CPU>(Instruction<T>);

impl<T: CPU> Default for InstructionBuilder<T> {
    fn default() -> Self {
        InstructionBuilder {
            0: Instruction::default(),
        }
    }
}

impl<T: CPU> InstructionBuilder<T> {
    pub fn build(self) -> Instruction<T> {
        self.0
    }

    pub fn set_condition(mut self, cond: u32) -> Self {
        self.0.condition = InstructionCondition::from(cond);
        self
    }

    pub fn set_fn(mut self, instr_fn: fn(&Instruction<T>, &mut T, &mut Memory)) -> Self {
        self.0.f = instr_fn;
        self
    }

    pub fn decode_data_processing(
        mut self,
        i: u32,
        s: u32,
        rn: u32,
        rd: u32,
        sr: u32,
    ) -> Self {
        let i = i != 0;
        let shift_op = get_shift_op(sr as u16, i);

        self.0.args = Some(InstructionArgs::DataProcessing(DataProcessingArgs {
            i,
            s: s != 0,
            rn,
            rd,
            shift_op
        }));
        self
    }

    pub fn decode_b_bl(mut self, l: u32, signed_immed_24: u32) -> Self {
        self.0.args = Some(InstructionArgs::Branch(l > 0, signed_immed_24));
        
        self
    }
}
