use rand::Rng;

pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    stack: [u16; 16],
    keys: [bool; 16],
    video: [u32; 64 * 32],

    // Indices
    index_register: u16,
    pc: u16,
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,

    rng: rand::rngs::ThreadRng,
}

#[allow(non_snake_case)]
impl Chip8 {
    const CODE_START_ADDRESS: usize = 0x200;
    const FONTSET_SIZE: usize = 80;
    const FONTSET_START_ADDRESS: usize = 0x50;
    const FONTSET: [u8; Chip8::FONTSET_SIZE] = [
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
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];
    const VIDEO_WIDTH: usize = 32;
    const VIDEO_HEIGHT: usize = 64;
    const VIDEO_SIZE: usize = Chip8::VIDEO_WIDTH * Chip8::VIDEO_HEIGHT;

    pub fn new(program: &Vec<u8>) -> Chip8 {
        let mut chip = Chip8 {
            registers: [0; 16],
            memory: [0; 4096],
            stack: [0; 16],
            keys: [false; 16],
            video: [0; Chip8::VIDEO_SIZE],

            index_register: 0,
            pc: Chip8::CODE_START_ADDRESS as u16,
            stack_pointer: 0, 
            delay_timer: 0,
            sound_timer: 0,

            rng: rand::thread_rng(),
        };

        // Load fonts
        for i in Chip8::FONTSET_START_ADDRESS..(Chip8::FONTSET_START_ADDRESS + Chip8::FONTSET_SIZE) {
            chip.memory[i] = Chip8::FONTSET[i];
        }

        // Load program
        for i in Chip8::CODE_START_ADDRESS..(Chip8::CODE_START_ADDRESS + program.len()) {
            chip.memory[i] = program[i];
        }

        chip
    }

    // INSTRUCTIONS ----------------------------------------------------------------
    // Description copied from wikipedia https://en.wikipedia.org/wiki/CHIP-8
    
    // Clears the screen.
    pub fn op_00E0(&mut self) {
        for i in 0..Chip8::VIDEO_SIZE {
            self.video[i] = 0;
        }
    }

    // 	Returns from a subroutine.
    pub fn op_00EE(&mut self) {
        if self.stack_pointer == 0 {
            panic!("00EE when sp is 0");
        }

        self.stack_pointer -= 1;
        self.pc = self.stack[self.stack_pointer as usize];
    }

    // Jumps to address NNN.
    pub fn op_1NNN(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    // Call subroutine at NNN
    pub fn op_2NNN(&mut self, nnn: u16) {
        self.stack[self.stack_pointer as usize] = self.pc;
        self.stack_pointer += 1;
        self.pc = nnn;
    }

    // Skips the next instruction if VX equals NN. (Usually the next instruction is a jump to skip a code block);
    pub fn op_3XNN(&mut self, vx: usize, nn: u8) {
        if self.registers[vx] == nn {
            self.pc += 2;
        }
    }

    // Skips the next instruction if VX does not equal NN. (Usually the next instruction is a jump to skip a code block);
    pub fn op_4XNN(&mut self, vx: usize, nn: u8) {
        if self.registers[vx] != nn {
            self.pc += 2;
        }
    }

    // Skips the next instruction if VX equals VY. (Usually the next instruction is a jump to skip a code block);
    pub fn op_5XY0(&mut self, vx: usize, vy: usize) {
        if self.registers[vx] == self.registers[vy] {
            self.pc += 2;
        }
    }

    // Sets VX to NN.
    pub fn op_6XNN(&mut self, vx: usize, nn: u8) {
        self.registers[vx] = nn;
    }

    // 	Adds NN to VX. (Carry flag is not changed);
    pub fn op_7XNN(&mut self, vx: usize, nn: u8) {
        self.registers[vx] += nn;
    }

    // Sets VX to the value of VY.
    pub fn op_8XY0(&mut self, vx: usize, nn: u8) {
        self.registers[vx] = nn;
    }

    // Sets VX to VX or VY. (Bitwise OR operation);
    pub fn op_8XY1(&mut self, vx: usize, nn: u8) {
        self.registers[vx] |= nn;
    }

    // Sets VX to VX and VY. (Bitwise AND operation);
    pub fn op_8XY2(&mut self, vx: usize, nn: u8) {
        self.registers[vx] &= nn;
    }

    // Sets VX to VX xor VY.
    pub fn op_8XY3(&mut self, vx: usize, nn: u8) {
        self.registers[vx] ^= nn;
    }

    // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there is not.
    pub fn op_8XY4(&mut self, vx: usize, vy: usize) {
        let added_u16: u16 = self.registers[vx] as u16 + self.registers[vy] as u16;
        if added_u16 >= 0x100 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx] = (added_u16 & 0xFF) as u8;
    }

    // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there is not.
    pub fn op_8XY5(&mut self, vx: usize, vy: usize) {
        if self.registers[vx] > self.registers[vy] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx] -= self.registers[vy];
    }

    // Stores the least significant bit of VX in VF and then shifts VX to the right by 1.[b]
    pub fn op_8XY6(&mut self, vx: usize) {
        self.registers[0xF] = self.registers[vx] & 0x1;
        self.registers[vx] >>= 1;
    }

    // Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there is not.
    pub fn op_8XY7(&mut self, vx: usize, vy: usize) {
        if self.registers[vy] > self.registers[vx] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[vx] = self.registers[vx] - self.registers[vy];
    }

    // Stores the most significant bit of VX in VF and then shifts VX to the left by 1.[b]
    pub fn op_8XYE(&mut self, vx: usize) {
        self.registers[0xF] = (self.registers[vx] & 0x8) >> 7;
        self.registers[vx] <<= 1;
    }

    // Skips the next instruction if VX does not equal VY. (Usually the next instruction is a jump to skip a code block);
    pub fn op_9XY0(&mut self, vx: usize, vy: usize) {
        if self.registers[vx] != self.registers[vy] {
            self.pc += 2;
        }
    }

    // Sets I to the address NNN.
    pub fn op_ANNN(&mut self, nnn: u16) {
        self.index_register = nnn;
    }

    // Jumps to the address NNN plus V0.
    pub fn op_BNNN(&mut self, nnn: u16) {
        self.pc = self.registers[0] as u16 + nnn;
    }

    // Sets VX to the result of a bitwise and operation on a random byte
    pub fn op_CXNN(&mut self, vx: usize) {
        let r: u8 = self.rng.gen();
        self.registers[vx] &= r;
    }

    // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels. 
    // Each row of 8 pixels is read as bit-coded starting from memory location I; 
    // I value does not change after the execution of this instruction. As described above, 
    // VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn, and to 0 if that does not happen
    pub fn op_DXYN(&mut self, vx: usize, vy: usize, n: u8) {
        // sprite is always 8 pixels wide
        // wrap

        let x = self.registers[vx] as usize % Chip8::VIDEO_WIDTH;
        let y = self.registers[vy] as usize % Chip8::VIDEO_HEIGHT;

        self.registers[0xF] = 0;
        for r in 0..n as usize {
            for c in 0..8 as usize {
                let sprite_pixel = self.memory[(self.index_register as usize + (r as usize * 8) + c as usize)];
                if sprite_pixel != 0 {
                    let screen_pixel_index = (y + r) as usize * Chip8::VIDEO_WIDTH + x + c;
                    if self.video[screen_pixel_index] != 0 {
                        // Flipped
                        self.registers[0xF] = 1;
                    }

                    self.video[screen_pixel_index] ^= 0xFFFFFFFF;
                }
            } 
        }
    }

    // Skips the next instruction if the key stored in VX is pressed. (Usually the next instruction is a jump to skip a code block);
    pub fn op_EX9E(&mut self, vx: usize) {
        if self.keys[self.registers[vx] as usize] {
            self.pc += 2;
        }
    }

    // Skips the next instruction if the key stored in VX is not pressed. (Usually the next instruction is a jump to skip a code block);
    pub fn op_EXA1(&mut self, vx: usize) {
        if !self.keys[self.registers[vx] as usize] {
            self.pc += 2;
        }
    }
    
    // Sets VX to the value of the delay timer.
    pub fn op_FX07(&mut self, vx: usize) {
        self.registers[vx] = self.delay_timer;
    }

    // A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event);
    pub fn op_FX0A(&mut self, vx: usize) {
        let mut pressed = false;
        for i in 0..16 as usize {
            if self.keys[i] {
                self.registers[vx] = i as u8;
                pressed = true;
                break;
            }
        }

        if !pressed {
            self.pc -= 2;
        }
    }

    // Sets the delay timer to VX.
    pub fn op_FX15(&mut self, vx: usize) {
        self.delay_timer = self.registers[vx];
    }

    // Sets the sound timer to VX.
    pub fn op_FX18(&mut self, vx: usize) {
        self.sound_timer = self.registers[vx];
    }

    // Adds VX to I. VF is not affected.
    pub fn op_FX1E(&mut self, vx: usize) {
        self.index_register += self.registers[vx] as u16;
    }

    // Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
    pub fn op_FX29(&mut self, vx: usize) {
        self.index_register = Chip8::FONTSET_START_ADDRESS as u16 + 5 * self.registers[vx] as u16;
    }

    // Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, 
    // the middle digit at I plus 1, and the least significant digit at I plus 2. 
    // (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, 
    // the tens digit at location I+1, and the ones digit at location I+2.);
    pub fn op_FX33(&mut self, vx: usize) {
        let mut val = self.registers[vx];
        self.memory[self.index_register as usize + 2] = val % 10;
        val /= 10;
        self.memory[self.index_register as usize + 1] = val % 10;
        val /= 10;
        self.memory[self.index_register as usize] = val % 10;
    }

    // Stores from V0 to VX (including VX) in memory, starting at address I. 
    // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
    pub fn op_FX55(&mut self, vx: usize) {
        for i in 0..vx+1 {
            self.memory[self.index_register as usize + i] = self.registers[i];
        }
    }

    // Fills from V0 to VX (including VX) with values from memory, starting at address I. 
    // The offset from I is increased by 1 for each value written, but I itself is left unmodified.
    pub fn op_FX65(&mut self, vx: usize) {
        for i in 0..vx+1 {
            self.registers[i] = self.memory[self.index_register as usize + i];
        }
    }
}
