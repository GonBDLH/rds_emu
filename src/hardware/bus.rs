use crate::hardware::cpu::{arm7::Arm7TDMI, arm9::Arm946ES};

use super::{cpu::interpret_arm, memory::Memory};

#[derive(Default)]
pub struct Bus {
    cpu_arm9: Arm946ES,
    cpu_arm7: Arm7TDMI,
    mem: Memory,
}

impl Bus {
    fn new() -> Self {
        Self::default()
    }

    pub fn cycle(&mut self) {
        interpret_arm(&mut self.cpu_arm9, &mut self.mem);
    }
}
