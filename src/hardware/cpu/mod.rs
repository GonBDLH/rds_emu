pub mod arm7;
pub mod arm9;
mod instructions;

use self::instructions::Instruction;
use super::memory::Memory;
use bitmatch::bitmatch;

#[derive(Default, Clone, Copy)]
pub enum CpuState {
    #[default]
    Arm,
    THUMB,
}

#[derive(Default)]
pub enum CpuMode {
    #[default]
    User,
    FIQ,
    IRQ,
    Supervisor,
    Abort,
    Undefined,
    System,
}

pub trait CPU {
    fn get_state(&self) -> CpuState;

    fn get_register(&self, reg: u32) -> u32;
    // fn get_register_ref_mut(&mut self, reg: usize) -> &mut u32;

    fn set_register(&mut self, reg: u32, val: u32);

    #[bitmatch]
    fn decode_arm<T: CPU>(cpu: &mut T, mem: &mut Memory, opcode: u32) -> Instruction<T>;
}

pub fn fetch_arm(cpu: &mut impl CPU, _mem: &Memory) -> u32 {
    // let regs = cpu.get_registers_mut();
    let dir = cpu.get_register(0) as usize;

    // regs.r15 += 4;
    cpu.set_register(0, dir as u32 + 4);

    // mem.read_32(dir)
    0b00110010101000110011000000011111
}

pub fn fetch_thumb(cpu: &mut impl CPU, mem: &Memory) -> u16 {
    // let regs = cpu.get_registers_mut();
    // let dir = regs.r15 as usize;

    let dir = cpu.get_register(0) as usize;

    // regs.r15 += 2;
    cpu.set_register(0, dir as u32 + 2);

    mem.read_16(dir)
}

pub fn interpret_arm<T: CPU>(cpu: &mut T, mem: &mut Memory) {
    let opcode = fetch_arm(cpu, mem);

    let instruction = T::decode_arm(cpu, mem, opcode);

    instruction.execute(cpu, mem);
}

#[derive(Default)]
struct Registers {
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r8_fiq: u32,
    r9: u32,
    r9_fiq: u32,
    r10: u32,
    r10_fiq: u32,
    r11: u32,
    r11_fiq: u32,
    r12: u32,
    r12_fiq: u32,
    r13: u32,
    r13_fiq: u32,
    r13_svc: u32,
    r13_abt: u32,
    r13_irq: u32,
    r13_und: u32,
    r14: u32,
    r14_fiq: u32,
    r14_svc: u32,
    r14_abt: u32,
    r14_irq: u32,
    r14_und: u32,
    r15: u32,
    cpsr: u32,
    spsr_fiq: u32,
    spsr_svc: u32,
    spsr_abt: u32,
    spsr_irq: u32,
    spsr_und: u32,
}
