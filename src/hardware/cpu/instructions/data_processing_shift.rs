use crate::hardware::{cpu::{arm9::CPSR, CPU}, memory::Memory};

use super::{Instruction, InstructionArgs};

pub fn logical_and<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn add_with_carry<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {
    let args = if let Some(args) = instr.args {
        if let InstructionArgs::DataProcessing(data_processing_args) = args {
            data_processing_args
        } else {
            unreachable!("WRONG ARGS FOR INSTRUCTION ADC")
        }
    } else {
        unreachable!("NO ARGS FOR INSTRUCTION ADC")
    };

    // TODO if condition met

    let shifter_operand = args.shift_op.execute(cpu);
    let cflag = (cpu.get_register(CPSR) >> 29) & 1;
    let rn_val = cpu.get_register(args.rn);
    
    let rd_val = rn_val.wrapping_add(shifter_operand).wrapping_add(cflag);
    cpu.set_register(args.rd, rd_val);
    if args.s && args.rd == 15 {
        // TODO
    } 

}

pub fn add<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn and<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn bic<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn cmn<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn cmp<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn eor<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn mov<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn mvn<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn orr<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn rsb<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn rsc<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn sbc<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn sub<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn teq<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub fn tst<T: CPU>(instr: &Instruction<T>, cpu: &mut T, mem: &mut Memory) {}

pub mod shifts {
    use crate::hardware::cpu::{arm9::CPSR, CPU};

    pub fn immediate<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let rotate_imm = (a >> 8) as u32;
        let immed_8 = (b as u8) as u32;

        let shifter_operand = immed_8.rotate_right(rotate_imm * 2);
        let shifter_carry_out = if rotate_imm == 0 {
            cflag
        } else {
            shifter_operand & (1 << 31) > 0
        };

        (shifter_operand, shifter_carry_out)
    }

    // Register
    pub fn register<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        (a as u32, cflag)
    }

    // Logical Shift Left by Immediate
    pub fn lsl_imm<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool;

        let shift_imm = a;
        let rm = cpu.get_register(b as usize);

        if shift_imm == 0 {
            shifter_operand = rm;
            shifter_carry_out = cflag;
        } else {
            shifter_operand = rm << shift_imm;
            shifter_carry_out = rm & (1 << (32 - shift_imm)) > 0;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Logical Shift Left by Register
    pub fn lsl_reg<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool;

        let rs = cpu.get_register(a as usize) as u8;
        let rm = cpu.get_register(b as usize);

        if rs == 0 {
            shifter_operand = rm;
            shifter_carry_out = cflag;
        } else if rs < 32 {
            shifter_operand = rm << rs;
            shifter_carry_out = rm & (1 << (32 - rs)) > 0;
        } else if rs == 32 {
            shifter_operand = 0;
            shifter_carry_out = rm & 1 > 0;
        } else {
            shifter_operand = 0;
            shifter_carry_out = false;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Logical Shift Right by Immediate
    pub fn lsr_imm<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool;

        let shift_imm = a;
        let rm = cpu.get_register(b as usize);

        if shift_imm == 0 {
            shifter_operand = 0;
            shifter_carry_out = rm & (1 << 31) > 0;
        } else {
            shifter_operand = rm >> shift_imm;
            shifter_carry_out = rm & (1 << (shift_imm - 1)) > 0;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Logical Shift Right by Register
    pub fn lsr_reg<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool;

        let rs = cpu.get_register(a as usize) as u8;
        let rm = cpu.get_register(b as usize);

        if rs == 0 {
            shifter_operand = rm;
            shifter_carry_out = cflag;
        } else if rs < 32 {
            shifter_operand = rm >> rs;
            shifter_carry_out = rm & (1 << (rs - 1)) > 0;
        } else if rs == 32 {
            shifter_operand = 0;
            shifter_carry_out = rm & (1 << 31) > 0;
        } else {
            shifter_operand = 0;
            shifter_carry_out = false;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Arithmetic Shift Right by Immediate
    pub fn asr_imm<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool;

        let shift_imm = a;
        let rm = cpu.get_register(b as usize);

        if shift_imm == 0 {
            if rm & (1 << 31) == 0 {
                shifter_operand = 0;
                shifter_carry_out = rm & (1 << 31) > 0;
            } else {
                shifter_operand = u32::MAX;
                shifter_carry_out = rm & (1 << 31) > 0;
            }
        } else {
            shifter_operand = ((rm as i32) >> shift_imm) as u32;
            shifter_carry_out = rm & (1 << (shift_imm - 1)) > 0;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Arithmetic Shift Right by Immediate
    pub fn asr_reg<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool;

        let rs = cpu.get_register(a) as u8;
        let rm = cpu.get_register(b);

        if rs == 0 {
            shifter_operand = rm;
            shifter_carry_out = false;
        } else if rs < 32 {
            shifter_operand = ((rm as i32) >> rs) as u32;
            shifter_carry_out = rm & (1 << (rs - 1)) > 0;
        } else {
            if rm & (1 << 31) > 0 {
                shifter_operand = 0;
            } else {
                shifter_operand = u32::MAX;
            }
            shifter_carry_out = rm & (1 << 31) > 0;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Rotate Right by Immediate
    // Rotate Right with extend
    pub fn ror_imm<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool; 

        let shift_imm = a;
        let rm = cpu.get_register(b as usize);
    
        if shift_imm == 0 {
            shifter_operand = ((cflag as u32) << 31) | (rm << 1);
            shifter_carry_out = rm & 1 > 0;
        } else {
            shifter_operand = rm.rotate_right(shift_imm as u32);
            shifter_carry_out = rm & (1 << (shift_imm - 1)) > 0;
        }

        (shifter_operand, shifter_carry_out)
    }

    // Rotate Right by Register
    pub fn ror_reg<T: CPU>(cpu: &T, a: u32, b: u32) -> (u32, bool) {
        let cflag = (cpu.get_register(CPSR) & (1 << 29)) > 0;

        let shifter_operand: u32;
        let shifter_carry_out: bool; 

        let rs = cpu.get_register(a as usize) as u8;
        let rm = cpu.get_register(b as usize);
    
        if rs == 0 {
            shifter_operand = rm;
            shifter_carry_out = cflag;
        } else if rs & 0x1F == 0 {
            shifter_operand = rm;
            shifter_carry_out = rm & (1 << 31) > 0;
        } else {
            shifter_operand = rm.rotate_right((rs & 0x1F) as u32);
            shifter_carry_out = rm & (1 << ((rs & 0x1F) - 1)) > 0;
        }

        (shifter_operand, shifter_carry_out)
    }
}
