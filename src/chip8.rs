 

struct Chip8 {
    registers: [u8; 8],
    memory: [u8; 4096],
    stack: [u16; 16],
    keys: [bool; 16],
    video: [u32; 64 * 32],

    // Indices
    index_register: u16,
    pc: u16,
    stack_pointer: u8,
    delay_timer: u8,
}

impl Chip8 {
    const CODE_START_ADDRESS: usize = 0x200;
}
