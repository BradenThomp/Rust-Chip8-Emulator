const FONT: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Processor {
    memory: [u8; 4096], // 4KB of memory.
    v: [u8; 16],        // 16 8-bit general purpose registers. V0 to VF
    i: u16,             // 16-bit I register.
    sound_timer: u8,    // 8-bit sound timer.
    delay_timer: u8,    // 8-bit delay timer.
    pc: u16,            // 16-bit program counter.
    sp: u8,             // 16-bit stack pointer.
    stack: [u16; 16],   // 16 16-bit value stack.
    vram: [u32; 64],    // 64x32 pixel monitor.
}

impl Processor {
    pub fn build() -> Processor {
        let mut memory: [u8; 4096] = [0x0; 4096];
        memory[0..80].copy_from_slice(&FONT);

        Processor {
            memory: memory,
            v: [0x0; 16],
            i: 0x0,
            sound_timer: 0x0,
            delay_timer: 0x0,
            pc: 0x0,
            sp: 0x0,
            stack: [0x0; 16],
            vram: [0x0; 64],
        }
    }
}

/// 0nnn - SYS addr
/// Jump to a machine code routine at nnn.
/// This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
fn inst_0nnn(cpu: &mut Processor, addr: u16) {}

/// 00E0 - CLS
/// Clear the display.
fn inst_00E0(cpu: &mut Processor) {
    cpu.vram = [0x0; 64];
}

/// 00EE - RET
/// Return from a subroutine.
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
fn inst_00EE(cpu: &Processor) {}

/// 1nnn - JP addr
/// Jump to location nnn.
/// The interpreter sets the program counter to nnn.
fn inst_1nnn(cpu: &Processor, addr: u16) {}

/// 2nnn - CALL addr
/// Call subroutine at nnn.
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
fn inst_2nnn(cpu: &Processor, addr: u16) {}

/// 3xkk - SE Vx, byte
/// Skip next instruction if Vx = kk.
/// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
fn inst_3xkk(cpu: &Processor, x: u8, kk: u8) {}

/// 4xkk - SNE Vx, byte
/// Skip next instruction if Vx != kk.
/// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
fn inst_4xkk(cpu: &Processor, x: u8, kk: u8) {}

/// 5xy0 - SE Vx, Vy
/// Skip next instruction if Vx = Vy.
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
fn inst_5xy0(cpu: &Processor, x: u8, y: u8) {}

/// 6xkk - LD Vx, byte
/// Set Vx = kk.
/// The interpreter puts the value kk into register Vx.
fn inst_6xkk(cpu: &Processor, x: u8, kk: u8) {}
