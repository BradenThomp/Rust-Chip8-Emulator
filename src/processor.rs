use rand::Rng;
use std::time;

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
    memory: [u8; 4096],             // 4KB of memory.
    v: [u8; 16],                    // 16 8-bit general purpose registers. V0 to VF
    i: u16,                         // 16-bit I register.
    sound_timer: u8,                // 8-bit sound timer.
    delay_timer: u8,                // 8-bit delay timer.
    pc: u16,                        // 16-bit program counter.
    sp: u8,                         // 16-bit stack pointer.
    stack: [u16; 16],               // 16 16-bit value stack.
    vram: [[u8; 64]; 32],           // 64x32 pixel monitor.
    prev_delay_tick: time::Instant, // The time at which the prev_delay_tick occured.
}

pub struct CycleResult {
    pub video_out: [[u8; 64]; 32], // 64x32 pixel monitor.
    pub video_changed: bool,
}

impl Processor {
    pub fn new() -> Processor {
        let mut memory: [u8; 4096] = [0x0; 4096];
        memory[0x50..0xa0].copy_from_slice(&FONT);

        Processor {
            memory: memory,
            v: [0x0; 16],
            i: 0x0,
            sound_timer: 0x0,
            delay_timer: 0x0,
            pc: 0x200,
            sp: 0x0,
            stack: [0x0; 16],
            vram: [[0x0; 64]; 32],
            prev_delay_tick: time::Instant::now(),
        }
    }

    pub fn load_cartridge(&mut self, cartridge: &[u8; 3584]) {
        self.memory[0x200..].clone_from_slice(&cartridge[..]);
    }

    pub fn cycle(&mut self, keyboard_input: [bool; 16]) -> CycleResult {
        if self.delay_timer > 0 {
            let now = time::Instant::now();
            let elapsed = now - self.prev_delay_tick;
            // decrease delay time every 1/60th of a second.
            if elapsed.as_millis() > 16 {
                self.delay_timer -= 1;
                self.prev_delay_tick = now;
            }
        }

        // Fetch
        let instruction: u16 = (self.memory[self.pc as usize] as u16) << 8
            | self.memory[(self.pc + 1) as usize] as u16;
        self.pc += 2;

        // Decode
        let inst_id = (instruction & 0xf000) >> 12;
        let x: u8 = ((instruction & 0x0f00) >> 8) as u8;
        let y: u8 = ((instruction & 0x00f0) >> 4) as u8;
        let nnn = instruction & 0x0fff;
        let nn: u8 = (instruction & 0x00ff) as u8;
        let n: u8 = (instruction & 0x000f) as u8;
        let mut vram_changed = false;
        // Execute
        match inst_id {
            0x0 => match nnn {
                0x0E0 => inst_00E0(self),
                0x0EE => inst_00EE(self),
                _ => inst_0nnn(self, nnn),
            },
            0x1 => inst_1nnn(self, nnn),
            0x2 => inst_2nnn(self, nnn),
            0x3 => inst_3xkk(self, x, nn),
            0x4 => inst_4xkk(self, x, nn),
            0x5 => inst_5xy0(self, x, y),
            0x6 => inst_6xkk(self, x, nn),
            0x7 => inst_7xkk(self, x, nn),
            0x8 => match n {
                0x0 => inst_8xy0(self, x, y),
                0x1 => inst_8xy1(self, x, y),
                0x2 => inst_8xy2(self, x, y),
                0x3 => inst_8xy3(self, x, y),
                0x4 => inst_8xy4(self, x, y),
                0x5 => inst_8xy5(self, x, y),
                0x6 => inst_8xy6(self, x, y),
                0x7 => inst_8xy7(self, x, y),
                0xE => inst_8xyE(self, x, y),
                _ => (),
            },
            0x9 => inst_9xy0(self, x, y),
            0xA => inst_Annn(self, nnn),
            0xB => inst_Bnnn(self, nnn),
            0xC => inst_Cxkk(self, x, nn),
            0xD => {
                vram_changed = inst_Dxyn(self, x as usize, y as usize, n as usize);
            }
            0xE => match nn {
                0x9E => inst_Ex9E(self, x, keyboard_input),
                0xA1 => inst_ExA1(self, x, keyboard_input),
                _ => (),
            },
            0xF => match nn {
                0x07 => inst_Fx07(self, x),
                0x0A => inst_Fx0A(self, x, keyboard_input),
                0x15 => inst_Fx15(self, x),
                0x18 => inst_Fx18(self, x),
                0x1E => inst_Fx1E(self, x),
                0x29 => inst_Fx29(self, x),
                0x33 => inst_Fx33(self, x),
                0x55 => inst_Fx55(self, x),
                0x65 => inst_Fx65(self, x),
                _ => (),
            },
            _ => (),
        }

        return CycleResult {
            video_out: self.vram,
            video_changed: vram_changed,
        };
    }
}

/// 0nnn - SYS addr
/// Jump to a machine code routine at nnn.
/// This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
fn inst_0nnn(cpu: &mut Processor, addr: u16) {}

/// 00E0 - CLS
/// Clear the display.
fn inst_00E0(cpu: &mut Processor) {
    cpu.vram = [[0x0; 64]; 32];
}

/// 00EE - RET
/// Return from a subroutine.
/// The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
fn inst_00EE(cpu: &mut Processor) {
    cpu.pc = cpu.stack[cpu.sp as usize];
    cpu.sp -= 1;
}

/// 1nnn - JP addr
/// Jump to location nnn.
/// The interpreter sets the program counter to nnn.
fn inst_1nnn(cpu: &mut Processor, nnn: u16) {
    cpu.pc = nnn;
}

/// 2nnn - CALL addr
/// Call subroutine at nnn.
/// The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
fn inst_2nnn(cpu: &mut Processor, nnn: u16) {
    cpu.sp += 1;
    cpu.stack[cpu.sp as usize] = cpu.pc;
    cpu.pc = nnn;
}

/// 3xkk - SE Vx, byte
/// Skip next instruction if Vx = kk.
/// The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
fn inst_3xkk(cpu: &mut Processor, x: u8, kk: u8) {
    let x = x as usize;
    if cpu.v[x as usize] == kk {
        cpu.pc += 2;
    }
}

/// 4xkk - SNE Vx, byte
/// Skip next instruction if Vx != kk.
/// The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
fn inst_4xkk(cpu: &mut Processor, x: u8, kk: u8) {
    let x = x as usize;
    if cpu.v[x] != kk {
        cpu.pc += 2;
    }
}

/// 5xy0 - SE Vx, Vy
/// Skip next instruction if Vx = Vy.
/// The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
fn inst_5xy0(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    if cpu.v[x] == cpu.v[y] {
        cpu.pc += 2;
    }
}

/// 6xkk - LD Vx, byte
/// Set Vx = kk.
/// The interpreter puts the value kk into register Vx.
fn inst_6xkk(cpu: &mut Processor, x: u8, kk: u8) {
    let x = x as usize;
    cpu.v[x] = kk;
}

/// 7xkk - ADD Vx, byte
/// Set Vx = Vx + kk.
/// Adds the value kk to the value of register Vx, then stores the result in Vx.
fn inst_7xkk(cpu: &mut Processor, x: u8, kk: u8) {
    let index = x as usize;
    let result: u16 = cpu.v[index] as u16 + kk as u16;
    cpu.v[index] = result as u8;
}

/// 8xy0 - LD Vx, Vy
/// Set Vx = Vy.
/// Stores the value of register Vy in register Vx.
fn inst_8xy0(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    cpu.v[x] = cpu.v[y];
}

/// 8xy1 - OR Vx, Vy
/// Set Vx = Vx OR Vy.
/// Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding
/// bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
fn inst_8xy1(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    cpu.v[x] = cpu.v[x] | cpu.v[y];
}

/// 8xy2 - AND Vx, Vy
/// Set Vx = Vx AND Vy.
/// Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding
/// bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
fn inst_8xy2(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    cpu.v[x] = cpu.v[x] & cpu.v[y];
}

/// 8xy3 - XOR Vx, Vy
/// Set Vx = Vx XOR Vy.
/// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the
/// corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1.
/// Otherwise, it is 0.
fn inst_8xy3(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    cpu.v[x] ^= cpu.v[y];
}

/// 8xy4 - ADD Vx, Vy
/// Set Vx = Vx + Vy, set VF = carry.
/// The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the
/// lowest 8 bits of the result are kept, and stored in Vx.
fn inst_8xy4(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    let result: u16 = (cpu.v[x] as u32 + cpu.v[y] as u32) as u16;
    cpu.v[0xF] = if result > 255 { 1 } else { 0 };
    cpu.v[x] = result as u8;
}

/// 8xy5 - SUB Vx, Vy
/// Set Vx = Vx - Vy, set VF = NOT borrow.
/// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
fn inst_8xy5(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    cpu.v[0xF] = if cpu.v[x] > cpu.v[y] { 1 } else { 0 };
    cpu.v[x] = cpu.v[x].wrapping_sub(cpu.v[y]);
}

/// 8xy6 - SHR Vx {, Vy}
/// Set Vx = Vx SHR 1.
/// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
fn inst_8xy6(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    cpu.v[0xF] = if (cpu.v[x] & 0x1) == 0x1 { 1 } else { 0 };
    cpu.v[x] /= 2;
}

/// 8xy7 - SUBN Vx, Vy
/// Set Vx = Vy - Vx, set VF = NOT borrow.
/// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
fn inst_8xy7(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    cpu.v[0xF] = if cpu.v[x] > cpu.v[y] { 1 } else { 0 };
    cpu.v[x] = cpu.v[y] - cpu.v[x];
}

/// 8xyE - SHL Vx {, Vy}
/// Set Vx = Vx SHL 1.
/// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
fn inst_8xyE(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    cpu.v[0xF] = if (cpu.v[x] & 0x80) == 0x80 { 1 } else { 0 };
    cpu.v[x] *= 2;
}

/// 9xy0 - SNE Vx, Vy
/// Skip next instruction if Vx != Vy.
/// The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
fn inst_9xy0(cpu: &mut Processor, x: u8, y: u8) {
    let x = x as usize;
    let y = y as usize;
    if cpu.v[x] != cpu.v[y] {
        cpu.pc += 2;
    }
}

/// Annn - LD I, addr
/// Set I = nnn.
/// The value of register I is set to nnn.
fn inst_Annn(cpu: &mut Processor, nnn: u16) {
    cpu.i = nnn;
}

/// Bnnn - JP V0, addr
/// Jump to location nnn + V0.
/// The program counter is set to nnn plus the value of V0.
fn inst_Bnnn(cpu: &mut Processor, nnn: u16) {
    cpu.pc = nnn + cpu.v[0] as u16;
}

/// Cxkk - RND Vx, byte
/// Set Vx = random byte AND kk.
/// The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
/// See instruction 8xy2 for more information on AND.
fn inst_Cxkk(cpu: &mut Processor, x: u8, kk: u8) {
    let x = x as usize;
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen();
    cpu.v[x] = number & kk;
}

/// Dxyn - DRW Vx, Vy, nibble
/// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
/// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at
/// coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it
/// is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side
/// of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen
/// and sprites.
fn inst_Dxyn(cpu: &mut Processor, x: usize, y: usize, n: usize) -> bool {
    cpu.v[0x0f] = 0;
    let mut vram_changed = false;
    for i in 0..n {
        let y = (cpu.v[y] as usize + i) % 32;
        for j in 0..8 {
            let x = (cpu.v[x] as usize + j) % 64;
            let bit = (cpu.memory[cpu.i as usize + i as usize] >> (7 - j)) & 0x01;
            cpu.v[0x0f] |= bit & cpu.vram[y][x];
            let prev_bit = cpu.vram[y][x];
            cpu.vram[y][x] ^= bit;
            vram_changed = vram_changed || (prev_bit != cpu.vram[y][x]);
        }
    }

    vram_changed
}

/// Ex9E - SKP Vx
/// Skip next instruction if key with the value of Vx is pressed.
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
fn inst_Ex9E(cpu: &mut Processor, x: u8, keyboard_input: [bool; 16]) {
    let x = x as usize;
    if keyboard_input[cpu.v[x] as usize] {
        cpu.pc += 2;
    }
}

/// ExA1 - SKNP Vx
/// Skip next instruction if key with the value of Vx is not pressed.
/// Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
fn inst_ExA1(cpu: &mut Processor, x: u8, keyboard_input: [bool; 16]) {
    let x = x as usize;
    if !keyboard_input[cpu.v[x] as usize] {
        cpu.pc += 2;
    }
}

/// Fx07 - LD Vx, DT
/// Set Vx = delay timer value.
/// The value of DT is placed into Vx.
fn inst_Fx07(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    cpu.v[x] = cpu.delay_timer;
}

/// Fx0A - LD Vx, K
/// Wait for a key press, store the value of the key in Vx.
/// All execution stops until a key is pressed, then the value of that key is stored in Vx.
fn inst_Fx0A(cpu: &mut Processor, x: u8, keyboard_input: [bool; 16]) {
    let x = x as usize;
    for i in 0..16 {
        if keyboard_input[i] {
            cpu.v[x] = i as u8;
            return;
        }
    }
    // Revert counter back to halt;
    cpu.pc -= 2;
}

/// Fx15 - LD DT, Vx
/// Set delay timer = Vx.
/// DT is set equal to the value of Vx.
fn inst_Fx15(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    cpu.delay_timer = cpu.v[x];
    cpu.prev_delay_tick = time::Instant::now();
}

/// Fx18 - LD ST, Vx
/// Set sound timer = Vx.
/// ST is set equal to the value of Vx.
fn inst_Fx18(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    cpu.sound_timer = cpu.v[x];
}

/// Fx1E - ADD I, Vx
/// Set I = I + Vx.
/// The values of I and Vx are added, and the results are stored in I.
fn inst_Fx1E(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    cpu.i += cpu.v[x] as u16;
}

/// Fx29 - LD F, Vx
/// Set I = location of sprite for digit Vx.
/// The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for
/// more information on the Chip-8 hexadecimal font.
fn inst_Fx29(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    cpu.i = cpu.v[x] as u16;
}

/// Fx33 - LD B, Vx
/// Store BCD representation of Vx in memory locations I, I+1, and I+2.
/// The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1,
/// and the ones digit at location I+2.
fn inst_Fx33(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    let address = cpu.i as usize;
    cpu.memory[address] = cpu.v[x] & 100;
    cpu.memory[address + 1] = cpu.v[x] & 10;
    cpu.memory[address + 2] = cpu.v[x] & 1;
}

/// Fx55 - LD [I], Vx
/// Store registers V0 through Vx in memory starting at location I.
/// The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
fn inst_Fx55(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    let address = cpu.i as usize;
    for i in 0..=x {
        cpu.memory[address + i] = cpu.v[i];
    }
}

/// Fx65 - LD Vx, [I]
/// Read registers V0 through Vx from memory starting at location I.
/// The interpreter reads values from memory starting at location I into registers V0 through Vx.
fn inst_Fx65(cpu: &mut Processor, x: u8) {
    let x = x as usize;
    let address = cpu.i as usize;
    for i in 0..=x {
        cpu.v[i] = cpu.memory[address + i];
    }
}
