use std::fmt::Debug;

pub const SCREEN_WIDTH: usize = 64; // 64 pixels wide
pub const SCREEN_HEIGHT: usize = 32; // 32 pixels tall

const RAM_SIZE: usize = 4096;
const NUM_REGISTERS: usize = 16; // 16 general-purpose registers
const STACK_SIZE: usize = 16; // Stack size for subroutine calls
const NUM_KEYS: usize = 16; // Number of keys (0x0 to 0xF)

const START_ADDRESS: u16 = 0x200; // Start of the program area in memory

const FONTSET_SIZE: usize = 80; // Size of the font set

const FONTSET: [u8; FONTSET_SIZE] = [
    // Fontset for digits 0-9 and letters A-F
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

pub struct Emu {
    pc: u16,                                      // Program Counter, 16 bit
    ram: [u8; RAM_SIZE],                          // 4K RAM
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT], // 64x32 pixel screen
    v_reg: [u8; NUM_REGISTERS],                   // 16 registers (V0 to VF)
    i_reg: u16,                                   // Index register
    sp: u16,                                      // Stack pointer
    stack: [u16; STACK_SIZE],                     // Stack for subroutine calls
    keys: [bool; NUM_KEYS],                       // Keypad state
    dt: u8,                                       // Delay timer
    st: u8,                                       // Sound timer
}

impl Emu {
    pub fn new() -> Self {
        let mut new_emu = Self {
            pc: START_ADDRESS,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; NUM_REGISTERS],
            i_reg: 0,
            sp: 0,
            stack: [0; STACK_SIZE],
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        new_emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        new_emu
    }

    fn push(&mut self, val: u16) {
        self.stack[self.sp as usize] = val;
        self.sp += 1;
    }

    fn pop(&mut self) -> u16 {
        self.sp -= 1;
        self.stack[self.sp as usize]
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDRESS;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGISTERS];
        self.i_reg = 0;
        self.sp = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        // Fetch
        let op = self.fetch();
        // Decode
        // Execute
    }

    pub fn fetch(&mut self) -> u16 {
        // Fetch the next opcode from memory
        let higher_byte = self.ram[self.pc as usize] as u16; // Es. 0x12
        let lower_byte = self.ram[(self.pc + 1) as usize] as u16; // Es. 0x34
        let op = (higher_byte << 8) | lower_byte; // Combine to form the opcode 0x1200 | 0x34 -> 0x1234
        self.pc += 2;
        op
    }

    pub fn execute(&mut self, op: u16) {
        let digit1 = (op & 0xF000) >> 12; // First nibble
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F; // Last nibble

        match (digit1, digit2, digit3, digit4) {
            (0, 0, 0, 0) => return,
            // CLS
            (0, 0, 0xE, 0) => {
                self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
            }
            // RET
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.pop();
                self.pc = ret_addr;
            }
            // JMP NNN
            (1, _, _, _) => {
                let nnn = op & 0xFFF;
                self.pc = nnn;
            }
            (_, _, _, _) => unimplemented!("Opcode {op:04X} not implemented yet"),
        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                // Play sound (this is a placeholder, actual sound handling would be more complex)
                println!("Beep!");
            }

            self.st -= 1;
        }
    }
}

impl Debug for Emu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Emu")
            .field("pc", &self.pc)
            .field("ram[..16]", &&self.ram[..16]) // show only first 16 bytes for brevity
            .field("screen[..16]", &&self.screen[..16]) // show only first 16 pixels
            .field("v_reg", &self.v_reg)
            .field("i_reg", &self.i_reg)
            .field("sp", &self.sp)
            .field("stack", &self.stack)
            .field("keys", &self.keys)
            .field("dt", &self.dt)
            .field("st", &self.st)
            .finish()
    }
}
