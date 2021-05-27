struct Processor {
    memory: [u8; 4096],      // 4KB of memory.
    gen_registers: [u8; 16], // 16 8-bit general purpose registers. V0 to VF
    i_register: u16,         // 16-bit I register.
    sound_timer: u8,         // 8-bit sound timer.
    delay_timer: u8,         // 8-bit delay timer.
    pc: u16,                 // 16-bit program counter.
    sp: u8,                  // 16-bit stack pointer.
    stack: [u16; 16],        // 16 16-bit value stack.
}
