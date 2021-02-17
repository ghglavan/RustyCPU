extern crate bytesize;

#[derive(Debug)]
pub struct StateSizes(pub usize);

impl StateSizes {
    pub const SYSTEM: StateSizes = StateSizes(16usize* bytesize::MB as usize); 
    pub const TIMER: StateSizes = StateSizes(16usize* bytesize::B as usize);  
    pub const KBD: StateSizes = StateSizes(256usize* bytesize::B as usize);   
    pub const NIC: StateSizes = StateSizes(4usize* bytesize::KB as usize);
    pub const SSD: StateSizes = StateSizes(1usize* bytesize::MB as usize);
    pub const GPU: StateSizes = StateSizes(512usize* bytesize::KB as usize);
    pub const RAM: StateSizes = StateSizes(14usize* bytesize::MB as usize); 
    // high-level code 
    pub const HLCODE: StateSizes = StateSizes(4usize* bytesize::KB as usize);
    pub const IVT: StateSizes = StateSizes(16usize* bytesize::B as usize);
    pub const HEAP: StateSizes = StateSizes(8usize* bytesize::MB as usize);
    pub const STACK: StateSizes = StateSizes(4usize* bytesize::MB as usize);
    pub const CODE: StateSizes = StateSizes(2usize* bytesize::MB as usize);

    pub(crate) const TOTAL: usize = (StateSizes::RAM.0 + StateSizes::TIMER.0 + StateSizes::KBD.0 + StateSizes::NIC.0 + StateSizes::SSD.0 + StateSizes::GPU.0) as usize;
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpCodes(u8);

impl OpCodes {
    pub const ADD: OpCodes= OpCodes(0);
    pub const SUB: OpCodes= OpCodes(1);
    pub const MUL: OpCodes= OpCodes(2);
    pub const DIV: OpCodes= OpCodes(3);
    pub const AND: OpCodes= OpCodes(4);
    pub const ORR: OpCodes= OpCodes(5);
    pub const EOR: OpCodes= OpCodes(6);
    pub const ASR: OpCodes= OpCodes(7);
    pub const ASL: OpCodes= OpCodes(8);
    pub const LDR: OpCodes= OpCodes(16);
    pub const STR: OpCodes= OpCodes(17);
    pub const BL: OpCodes= OpCodes(18);
    pub const BX: OpCodes= OpCodes(19);
    pub const PUSH: OpCodes= OpCodes(20);
    pub const POP: OpCodes= OpCodes(21);
    pub const MOV: OpCodes= OpCodes(22);
    pub const SET: OpCodes= OpCodes(23);
    pub const NOP: OpCodes= OpCodes(24);
    pub const CBZ: OpCodes= OpCodes(25);
    pub const CBNZ: OpCodes= OpCodes(26);
    pub const B: OpCodes= OpCodes(27);
    pub const ADR: OpCodes= OpCodes(28);
    pub const WFI: OpCodes= OpCodes(29);
    pub const HLI: OpCodes= OpCodes(30);
    pub const MSR: OpCodes= OpCodes(31);
}

impl std::convert::From<u8> for OpCodes {
    fn from(i: u8) -> OpCodes {
        OpCodes(i)
    }
}

#[derive(Debug)]
pub struct Registers(pub u8);

impl Registers {
    pub const R0: Registers = Registers(0);
    pub const R1: Registers = Registers(1);
    pub const R2: Registers = Registers(2);
    pub const R3: Registers = Registers(3);
    pub const R4: Registers = Registers(4);
    pub const R5: Registers = Registers(5);
    pub const R6: Registers = Registers(6);
    pub const R7: Registers = Registers(7);
    pub const R8: Registers = Registers(8);
    pub const R9: Registers = Registers(9);
    pub const SB: Registers = Registers(9);
    pub const R10: Registers = Registers(10);
    pub const SL: Registers = Registers(10);
    pub const R11: Registers = Registers(11);
    pub const FP: Registers = Registers(11);
    pub const R12: Registers = Registers(12);
    pub const SP: Registers = Registers(13);
    pub const LR: Registers = Registers(14);
    pub const PC: Registers = Registers(15);
    pub const PSR: Registers = Registers(16);
    pub const PRIMASK: Registers = Registers(17);
    pub const FAULTMASK: Registers = Registers(18);
    pub const BASEPRI: Registers = Registers(19);
    pub const CONTROL: Registers = Registers(20);
    pub const MSP: Registers = Registers(21);
    pub const PSP: Registers = Registers(22);

    pub(crate) const TOTAL : u8 = 22;
}

impl std::convert::From<u8> for Registers {
    fn from(i: u8) -> Registers {
        Registers(i)
    }
}

pub struct Address(pub u8);

#[derive(Debug)]
pub struct MemoryMap(pub usize);

impl MemoryMap {
    pub const IVT: MemoryMap = MemoryMap(0x0);
    pub const HEAP: MemoryMap = MemoryMap(MemoryMap::IVT.0 + StateSizes::IVT.0);
    pub const STACK: MemoryMap = MemoryMap(MemoryMap::HEAP.0 + StateSizes::HEAP.0);
    pub const CODE: MemoryMap = MemoryMap(MemoryMap::STACK.0 + StateSizes::STACK.0);
    pub const MEMTOP: MemoryMap = MemoryMap(MemoryMap::CODE.0 + StateSizes::CODE.0);
    pub const TIMER: MemoryMap = MemoryMap(MemoryMap::MEMTOP.0);
    pub const KBD: MemoryMap = MemoryMap(MemoryMap::TIMER.0 + StateSizes::TIMER.0);
    pub const NIC: MemoryMap = MemoryMap(MemoryMap::KBD.0 + StateSizes::KBD.0);
    pub const SSD: MemoryMap = MemoryMap(MemoryMap::NIC.0 + StateSizes::NIC.0);
    pub const GPU: MemoryMap = MemoryMap(MemoryMap::SSD.0 + StateSizes::SSD.0);
    pub const STATEOP: MemoryMap = MemoryMap(MemoryMap::GPU.0 + StateSizes::GPU.0);
}

