use crate::hardware::memory::Memory;

use super::instructions::branch::*;
use super::instructions::coprocessor_data_processing::*;
use super::instructions::coprocessor_ls_doublereg::*;
use super::instructions::coprocessor_reg_transfer::*;
use super::instructions::data_processing::*;
use super::instructions::extra_load_store::*;
use super::instructions::load_store::*;
use super::instructions::load_store_multiple::*;
use super::instructions::miscelaneous::*;
use super::instructions::multiplies::*;
use super::instructions::sw_int::*;
use super::instructions::unconditional::*;
use super::instructions::{data_processing_shift::*, InstructionBuilder};
use super::{instructions::Instruction, CpuState, CPU, CpuMode, Registers};

use bitmatch::bitmatch;

pub const R0: u32 = 0;
pub const R1: u32 = 1;
pub const R2: u32 = 2;
pub const R3: u32 = 3;
pub const R4: u32 = 4;
pub const R5: u32 = 5;
pub const R6: u32 = 6;
pub const R7: u32 = 7;
pub const R8: u32 = 8;
pub const R9: u32 = 9;
pub const R10: u32 = 10;
pub const R11: u32 = 11;
pub const R12: u32 = 12;
pub const R13: u32 = 13;
pub const R14: u32 = 14;
pub const R15: u32 = 15;
pub const CPSR: u32 = 16;
pub const SPSR: u32 = 17;

#[derive(Default)]
pub struct Arm946ES {
    state: CpuState,
    regs: Registers,
    mode: CpuMode,
}

impl CPU for Arm946ES {
    fn get_state(&self) -> CpuState {
        self.state
    }

    fn get_register(&self, reg: u32) -> u32 {
        match reg {
            0 => self.regs.r0,
            1 => self.regs.r1,
            2 => self.regs.r2,
            3 => self.regs.r3,
            4 => self.regs.r4,
            5 => self.regs.r5,
            6 => self.regs.r6,
            7 => self.regs.r7,
            8 => match self.mode {
                CpuMode::FIQ => self.regs.r8_fiq,
                _ => self.regs.r8
            },
            9 => match self.mode {
                CpuMode::FIQ => self.regs.r9_fiq,
                _ => self.regs.r9
            }
            10 => match self.mode {
                CpuMode::FIQ => self.regs.r9_fiq,
                _ => self.regs.r9
            }
            11 => match self.mode {
                CpuMode::FIQ => self.regs.r10_fiq,
                _ => self.regs.r10
            }
            12 => match self.mode {
                CpuMode::FIQ => self.regs.r11_fiq,
                _ => self.regs.r11
            }
            13 => match self.mode {
                CpuMode::Supervisor => self.regs.r13_svc,
                CpuMode::Abort => self.regs.r13_abt,
                CpuMode::Undefined => self.regs.r13_und,
                CpuMode::IRQ => self.regs.r13_irq,
                CpuMode::FIQ => self.regs.r13_fiq,
                _ => self.regs.r13
            }
            14 => match self.mode {
                CpuMode::Supervisor => self.regs.r14_svc,
                CpuMode::Abort => self.regs.r14_abt,
                CpuMode::Undefined => self.regs.r14_und,
                CpuMode::IRQ => self.regs.r14_irq,
                CpuMode::FIQ => self.regs.r14_fiq,
                _ => self.regs.r14
            }
            15 => self.regs.r15,
            16 => self.regs.cpsr,
            17 => match self.mode {
                CpuMode::Supervisor => self.regs.spsr_svc,
                CpuMode::Abort => self.regs.spsr_abt,
                CpuMode::Undefined => self.regs.spsr_und,
                CpuMode::IRQ => self.regs.spsr_irq,
                CpuMode::FIQ => self.regs.spsr_fiq,
                _ => unreachable!("SPSR En modo User o System")
            }

            _ => unreachable!("Registro no existente")
        }
    }

    // fn get_register_ref_mut(&mut self, reg: usize) -> &mut u32 {
    //     &mut self.regs.r1
    // }

    fn set_register(&mut self, reg: u32, val: u32) {
        match reg {
            0 => self.regs.r0 = val,
            1 => self.regs.r1 = val,
            2 => self.regs.r2 = val,
            3 => self.regs.r3 = val,
            4 => self.regs.r4 = val,
            5 => self.regs.r5 = val,
            6 => self.regs.r6 = val,
            7 => self.regs.r7 = val,
            8 => match self.mode {
                CpuMode::FIQ => self.regs.r8_fiq = val,
                _ => self.regs.r8 = val
            },
            9 => match self.mode {
                CpuMode::FIQ => self.regs.r9_fiq = val,
                _ => self.regs.r9 = val
            }
            10 => match self.mode {
                CpuMode::FIQ => self.regs.r9_fiq = val,
                _ => self.regs.r9 = val
            }
            11 => match self.mode {
                CpuMode::FIQ => self.regs.r10_fiq = val,
                _ => self.regs.r10 = val
            }
            12 => match self.mode {
                CpuMode::FIQ => self.regs.r11_fiq = val,
                _ => self.regs.r11 = val
            }
            13 => match self.mode {
                CpuMode::Supervisor => self.regs.r13_svc = val,
                CpuMode::Abort => self.regs.r13_abt = val,
                CpuMode::Undefined => self.regs.r13_und = val,
                CpuMode::IRQ => self.regs.r13_irq = val,
                CpuMode::FIQ => self.regs.r13_fiq = val,
                _ => self.regs.r13 = val
            }
            14 => match self.mode {
                CpuMode::Supervisor => self.regs.r14_svc = val,
                CpuMode::Abort => self.regs.r14_abt = val,
                CpuMode::Undefined => self.regs.r14_und = val,
                CpuMode::IRQ => self.regs.r14_irq = val,
                CpuMode::FIQ => self.regs.r14_fiq = val,
                _ => self.regs.r14 = val
            }
            15 => self.regs.r15 = val,
            16 => self.regs.cpsr = val,
            17 => match self.mode {
                CpuMode::Supervisor => self.regs.spsr_svc = val,
                CpuMode::Abort => self.regs.spsr_abt = val,
                CpuMode::Undefined => self.regs.spsr_und = val,
                CpuMode::IRQ => self.regs.spsr_irq = val,
                CpuMode::FIQ => self.regs.spsr_fiq = val,
                _ => unreachable!("SPSR En modo User o System")
            }

            _ => unreachable!("Registro no existente")
        }
    }

    

    #[allow(non_snake_case)]
    #[bitmatch]
    fn decode_arm<T: CPU>(cpu: &mut T, mem: &mut Memory, opcode: u32) -> Instruction<T> {
        #[bitmatch]
        match opcode {
            "aaaa00b0101cddddeeeeffffffffffff" => InstructionBuilder::default()
                .set_condition(a)
                .set_fn(add_with_carry)
                .decode_data_processing(b, c, d, e, f)
                .build(),

            "aaaa00b0100cddddeeeeffffffffffff" => InstructionBuilder::default()
                .set_condition(a)
                .set_fn(add)
                .decode_data_processing(b, c, d, e, f)
                .build(),

            "aaaa00b0000cddddeeeeffffffffffff" => InstructionBuilder::default()
                .set_condition(a)
                .set_fn(and)
                .decode_data_processing(b, c, d, e, f)
                .build(),

            "aaaa101bcccccccccccccccccccccccc" => InstructionBuilder::default()
                .set_condition(a)
                .set_fn(b_bl)
                .decode_b_bl(b, c)
                .build(),
                
            // "cccc00i1110Snnnnddddssssssssssss" => Instruction {f: bic },
            // "111000010010iiiiiiiiiiii0111IIII" => Instruction {f: bkpt },
            // "1111101Hiiiiiiiiiiiiiiiiiiiiiiii" => Instruction {f: blx_unconditional },
            // "cccc00010010aaaabbbbdddd0011mmmm" => Instruction {f: blx_miscelaneous },
            // "cccc00010010aaaabbbbdddd0001mmmm" => Instruction {f: bx },
            // "cccc00010010aaaabbbbdddd0010mmmm" => Instruction {f: bxj },
            // "cccc1110aaaabbbbddddeeeefff0gggg" => Instruction {f: cdp },
            // "cccc00010110aaaabbbbdddd0001eeee" => Instruction {f: clz },
            // "cccc00i10111aaaabbbbssssssssssss" => Instruction {f: cmn },
            // "cccc00i10101aaaabbbbssssssssssss" => Instruction {f: cmp },
            // "111100010000aab0cccccccdef0ggggg" => Instruction {f: cps },
            // "cccc00i0001Saaaabbbbssssssssssss" => Instruction {f: eor },
            // "cccc110punw1aaaabbbbddddoooooooo" => Instruction {f: ldc },
            // "cccc100punw1aaaabbbbbbbbbbbbbbbb" => Instruction {f: ldm_1 },
            // "cccc100pu101aaaa0bbbbbbbbbbbbbbb" => Instruction {f: ldm_2 },
            // "cccc100pu1w1aaaa1bbbbbbbbbbbbbbb" => Instruction {f: ldm_3 },
            // "cccc01ipu0w1aaaabbbbdddddddddddd" => Instruction {f: ldr },
            // "cccc01ipu1w1aaaabbbbdddddddddddd" => Instruction {f: ldrb },
            // "cccc01i0u111aaaabbbbdddddddddddd" => Instruction {f: ldrbt },
            // "cccc000puIw0aaaabbbbdddd1101eeee" => Instruction {f: ldrd },
            // "cccc000puIw1aaaabbbbdddd1011eeee" => Instruction {f: ldrh },
            // "cccc000puIw1aaaabbbbdddd1101eeee" => Instruction {f: ldrsb },
            // "cccc000puIw1aaaabbbbdddd1111eeee" => Instruction {f: ldrsh },
            // "cccc01i0u011aaaabbbbdddddddddddd" => Instruction {f: ldrt },
            // "cccc1110aaa0bbbbddddeeeefff1gggg" => Instruction {f: mcr },
            // "cccc11000100aaaabbbbddddeeeeffff" => Instruction {f: mcrr },
            // "cccc0000001saaaabbbbdddd1001eeee" => Instruction {f: mla },
            // "cccc00i1101Saaaabbbbssssssssssss" => Instruction {f: mov },
            // "cccc1110aaa1bbbbddddeeeefff1gggg" => Instruction {f: mrc },
            // "cccc11000101aaaabbbbddddeeeeffff" => Instruction {f: mrrc },
            // "cccc00010r00aaaabbbbdddddddddddd" => Instruction {f: mrs },
            // "cccc00110r10aaaabbbbddddeeeeeeee" => Instruction {f: msr_imm },
            // "cccc00010r10aaaabbbbdddd0000eeee" => Instruction {f: msr_reg },
            // "cccc0000000saaaabbbbdddd1001eeee" => Instruction {f: mul },
            // "cccc00i1111Saaaabbbbssssssssssss" => Instruction {f: mvn },
            // "cccc00i1100Saaaabbbbssssssssssss" => Instruction {f: orr },
            // "111101i1u101aaaa1111bbbbbbbbbbbb" => Instruction {f: pld },
            // "cccc00010000aaaabbbbdddd0101dddd" => Instruction {f: qadd },
            // "cccc00010100aaaabbbbdddd0101dddd" => Instruction {f: qdadd },
            // "cccc00010110aaaabbbbdddd0101dddd" => Instruction {f: qdsub },
            // "cccc00010010aaaabbbbdddd0101dddd" => Instruction {f: qsub },
            // "cccc00i0011Saaaabbbbssssssssssss" => Instruction {f: rsb },
            // "cccc00i0111Saaaabbbbssssssssssss" => Instruction {f: rsc },
            // "cccc00i0110Saaaabbbbssssssssssss" => Instruction {f: sbc },
            // "cccc00010000aaaabbbbdddd1yx0eeee" => Instruction {f: smlaxy },
            // "cccc0000111Saaaabbbbdddd1001eeee" => Instruction {f: smlal },
            // "cccc00010100aaaabbbbdddd1yx0eeee" => Instruction {f: smlalxy },
            // "cccc00010010aaaabbbbdddd1y00eeee" => Instruction {f: smlawy },
            // "cccc00010110aaaabbbbdddd1yx0eeee" => Instruction {f: smulxy },
            // "cccc0000110Saaaabbbbdddd1001eeee" => Instruction {f: smull },
            // "cccc00010010aaaabbbbdddd1y10eeee" => Instruction {f: smulwy },
            // "cccc110punw0aaaabbbbddddeeeeeeee" => Instruction {f: stc },
            // "1111110punw0aaaabbbbddddeeeeeeee" => Instruction {f: stc_2 },
            // "cccc100pu0w0aaaabbbbbbbbbbbbbbbb" => Instruction {f: stm },
            // "cccc100pu100aaaabbbbbbbbbbbbbbbb" => Instruction {f: stm_2 },
            // "cccc01ipu0w0aaaabbbbeeeeeeeeeeee" => Instruction {f: str_fn },
            // "cccc01ipu1w0aaaabbbbeeeeeeeeeeee" => Instruction {f: strb },
            // "cccc01i0u110aaaabbbbeeeeeeeeeeee" => Instruction {f: strbt },
            // "cccc000puIw0aaaabbbbdddd1111eeee" => Instruction {f: strd },
            // "cccc000puIw0aaaabbbbdddd1011eeee" => Instruction {f: strh },
            // "cccc01i0u010aaaabbbbeeeeeeeeeeee" => Instruction {f: strt },
            // "cccc00i0010Saaaabbbbssssssssssss" => Instruction {f: sub },
            // "cccc1111iiiiiiiiiiiiiiiiiiiiiiii" => Instruction {f: swi },
            // "cccc00010000aaaabbbbdddd1001eeee" => Instruction {f: swp },
            // "cccc00010100aaaabbbbdddd1001eeee" => Instruction {f: swpb },
            // "cccc00i10011aaaabbbbssssssssssss" => Instruction {f: teq },
            // "cccc00i10001aaaabbbbssssssssssss" => Instruction {f: tst },
            // "cccc0000101Saaaabbbbdddd1001eeee" => Instruction {f: umlal },
            // "cccc0000100Saaaabbbbdddd1001eeee" => Instruction {f: umull },
            _ => InstructionBuilder::default().build(),
        }
    }
}
