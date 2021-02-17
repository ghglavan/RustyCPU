use crate::constants::*;
use crate::instructions::*;

pub struct Cpu {
    system_state: Vec<u8>,
    registers: Vec<Registers>, 
}

impl Cpu {
    pub fn new() -> Self {
        let mut system_state = Vec::with_capacity(StateSizes::TOTAL);
        system_state.resize(StateSizes::TOTAL, 0);
        
        Cpu {
            system_state: system_state,
            registers: (0u8..=Registers::TOTAL).map(Registers::from).collect::<Vec<Registers>>(),
        }
    }
    
    fn execute_instruction(&mut self, instruction: Instructions) {
        match instruction.opcode() {
            &OpCodes::LDR => {
                self.registers[instruction.r1().0 as usize].0 = self.system_state[instruction.addr().0 as usize]
            },
            &OpCodes::STR => {
                self.system_state[instruction.addr().0 as usize] = self.registers[instruction.r1().0 as usize].0
            },
            &OpCodes::B => {
                self.registers[Registers::PC.0 as usize].0 = instruction.addr().0
            },
            &OpCodes::CBZ => {
                if self.registers[instruction.r1().0 as usize].0 == 0 {
                    self.registers[Registers::PC.0 as usize].0 = instruction.addr().0;
                }
            },
            &OpCodes::CBNZ => {
                if self.registers[instruction.r1().0 as usize].0 != 0 {
                    self.registers[Registers::PC.0 as usize].0 = instruction.addr().0;
                }
            },
            // todo: implement the other operations
            _ => panic!("not implemented!"),
        } 
    }

    pub fn tick(&mut self) {
        // normally, we should tick every peripheral and allow for irqs.
        // something like:
        // let irq = kbdAction(self.system_state)
        // but since we only want to construct the cpu, we can ignore them :)
        
        let instruction = self.fetch_instruction();
        self.execute_instruction(instruction);
        
    }

    fn next_pc_byte(&mut self) -> u8 {
        self.registers[Registers::PC.0 as usize].0 += 1;
        self.system_state[self.registers[Registers::PC.0 as usize].0 as usize - 1]
    }

    fn fetch_instruction(&mut self) -> Instructions {
        let instruction =  self.next_pc_byte();
        
        match OpCodes::from(instruction) {
            OpCodes::LDR | OpCodes::STR | OpCodes::MOV | OpCodes::SET | OpCodes::CBZ | OpCodes::CBNZ => {
                let r1 = self.next_pc_byte();
                let addr = self.next_pc_byte(); 

                Instructions::TwoArgs(OpCodes::from(instruction), Registers(r1), Address(addr))
            },
            OpCodes::B | OpCodes::BL | OpCodes::BX => {
                let addr = self.next_pc_byte(); 

                Instructions::OneArg(OpCodes::from(instruction), Address(addr))
            },
            OpCodes::NOP | OpCodes::WFI => Instructions::NoArgs(OpCodes::from(instruction)),
            _ => {
                let r1 = self.next_pc_byte(); 
                let r2 = self.next_pc_byte(); 
                let r3 = self.next_pc_byte();

                Instructions::ThreeArgs(OpCodes::from(instruction), Registers(r1), Registers(r2), Registers(r3))
            }
        }
    }
}
