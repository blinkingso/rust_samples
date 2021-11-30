//! rust data types in computer;
//! Annotation allow(arithmetic_overflow) Required declaration.
//! The Rust compiler can detect this obvious overflow situation.
#[allow(arithmetic_overflow)]
fn main() {
    {
        let a: u16 = 50115;
        let b: i16 = -15421;
        println!("a: {:016b} {}", a, a);
        println!("b: {:016b} {}", b, b);
    }

    {
        let a: f32 = 42.42;
        // transmute 42.42 bits pattern to u32 without affecting any of the underlying bits;
        let frankentype: u32 = unsafe { std::mem::transmute(a) };
        // 32-bit binary representation for 42.42;
        println!("{}, {:032b}", frankentype, frankentype);
        // transmute 1110027796u32 to 42.42 with 01000010001010011010111000010100b;
        let b: f32 = unsafe { std::mem::transmute(frankentype) };
        println!("{}", b);
        assert_eq!(a, b);
    }

    // {
    //     // integer overflow;
    //     let mut i = 0_u16;
    //     println!("{}..", i);

    //     // rustc -O ch_05.rs and execute makes some different;
    //     // no integer overflow happens. when -O used.
    //     loop {
    //         i += 1;
    //         print!("{}..", i);
    //         if i % 10000 == 0 {
    //             print! {"\n"};
    //         }
    //     }
    // }

    {
        let zero = 0b0_u16;
        let one = 0b1_u16;
        let two = 0b10_u16;
        //....
        let s_65535 = 0b1111_1111_1111_1111_u16;
        println!("{},{},{},{}", zero, one, two, s_65535);
    }
    // {
    //     // overflow;
    //     let (a, b) = (200, 200);
    //     let c: u8 = a + b;
    //     println!("200 + 200 = {}", c);
    // }

    {
        let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
        let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];
        let a: i32 = unsafe { std::mem::transmute(big_endian) };
        let b: i32 = unsafe { std::mem::transmute(little_endian) };
        println!("{} vs {}", a, b);
        // u32 binary pattern;
        let binary = unsafe { std::mem::transmute::<f32, u32>(1.325625) };
        // 01010011010111000010100
        println!("1.32525's binary pattern is : {}, {:032b}", binary, binary);
        let man = unsafe { std::mem::transmute::<u32, u32>(0b01010011010111000010100) };
        println!("{}", man);
        let b = unsafe { std::mem::transmute::<f32, u32>(42.42) };
        println!("42.42's binary pattern is : {}, {:032b}", b, b);
        println!("{}", unsafe { std::mem::transmute::<u32, f32>(2731540) });
    }

    {
        // isolating the sign bit;
        let n: f32 = -42.42333333;
        let n_bits: u32 = n.to_bits();
        let sign_bit = n_bits >> 31;
        println!("sign bit {}", sign_bit);
        // isolating the exponent
        let exponent_bit = n_bits >> 23 & 0xff;
        println!("exponent {}", exponent_bit);
        let overflow: f32 = (2u64.pow(32) - 1) as f32;
        let overflow = overflow * 2f32;
        println!("overflow {}", overflow);
        // isolate the mantissa;
        println!("{:032x}", 0b11111111111111111111111);
        let mantissa = n_bits & 0x7fffff;
        println!("mantissa is {}", mantissa);
        let mut mantissa = 1.0f32;
        for i in 0..=22 {
            let mask = 1 << i;
            let one_at_bit_i = n_bits & mask;
            if one_at_bit_i != 0 {
                let i_ = i as f32;
                let weight = 2_f32.powf(i_ - 23.0);
                mantissa += weight;
            }
        }

        println!("full mantissa is : {}", mantissa);

        println!("{}, {}", (-1.0f32).powf(0.0), -1.0f32.powf(0.0));
    }
}
// Definition of the Q7 format; Q7 represent -1..1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q7(i8);

impl From<f64> for Q7 {
    fn from(value: f64) -> Self {
        if value >= 1.0 {
            Q7(127)
        } else if value <= -1.0 {
            Q7(-128)
        } else {
            // loses accuracy;
            Q7((value * 128.0) as i8)
        }
    }
}

impl From<Q7> for f64 {
    fn from(value: Q7) -> Self {
        // q.0 / 128;
        (value.0 as f64) * 2_f64.powf(-7.0)
    }
}

impl From<f32> for Q7 {
    fn from(value: f32) -> Self {
        Q7::from(value as f64)
    }
}

impl From<Q7> for f32 {
    fn from(value: Q7) -> Self {
        f64::from(value) as f32
    }
}

#[test]
fn out_of_bounds() {
    assert_eq!(Q7::from(10.), Q7::from(1.));
    assert_eq!(Q7::from(-10.), Q7::from(-1.));
    assert_eq!("10", "10");
}

#[test]
fn f32_to_q7() {
    let n1 = 0.7_f32;
    let q1 = Q7::from(n1);

    let n2 = -0.4;
    let q2 = Q7::from(n2);

    let n3 = 123.0;
    let q3 = Q7::from(n3);

    assert_eq!(q1, Q7(89));
    assert_eq!(q2, Q7(-51));
    assert_eq!(q3, Q7(127));
}

/// generating `f32` values in interval [0,1] from a `u8` with division;
pub fn mock_rand(n: u8) -> f32 {
    (n as f32) / 255.0
}

pub const BASE: u32 = 0b0_01111110_00000000000000000000000;
/// mock rand2 optimised;
pub fn mock_rand2(n: u8) -> f32 {
    println!("BASE IS: {}", BASE >> 23);
    let large_n = (n as u32) << 15;
    let f32_bits = BASE | large_n;
    let m = f32::from_bits(f32_bits);
    println!("m is {}", m);
    2.0 * (m - 0.5)
}

#[test]
fn random_bytes() {
    // let rand = 0x00;
    let rand = 0x7f;
    // let rand = 0xff;
    let (a1, a2) = (mock_rand(rand), mock_rand2(rand));
    println!("{} vs {}", a1, a2);
}

// 5.7 Implementing a CPU to establish that functions are also data.
struct CPU {
    /// all CHIP-8 opcodes are u16 values.
    // current_operation: u16,
    /// for addtion registers.
    registers: [u8; 16],
    memory: [u8; 4096],
    position_in_memory: usize,
    stack: [u16; 16],
    // easier to index values within the stack.
    stack_pointer: usize,
}

impl CPU {
    /// reads ops from memory.
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_b1 = self.memory[p] as u16;
        let op_b2 = self.memory[p + 1] as u16;
        // create a full u16 instruction
        let opcode = op_b1 << 8 | op_b2;
        println!("opcode is : {}, position in memory is : {}", opcode, &p);
        opcode
    }

    /// add ops, add data at position x and data at position y. and set to x position;
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        // overflow and save `state` to the last register;
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    /// cpu CALL instruction
    fn call(&mut self, addr: u16) {
        println!(
            "call invoked here, stack_pointer is : {}, addr is {}, stack is: {:?}",
            self.stack_pointer, addr, self.stack
        );
        let stack_pointer = self.stack_pointer;
        let stack = &mut self.stack;
        if stack_pointer > stack.len() {
            panic!("Stack overflow");
        }
        stack[stack_pointer] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    /// cpu loop running
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = ((opcode & 0x000F) >> 0) as u8;

            // calculate the function's memory address
            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                // indicate the add operation
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
}

#[test]
fn test_cpu() {
    let mut cpu = CPU {
        position_in_memory: 0,
        registers: [0; 16],
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    // data in register;
    cpu.registers[0] = 5;
    cpu.registers[1] = 20;
    // cpu.registers[2] = 10;
    // cpu.registers[3] = 0;

    // instructions in memory;
    let mem = &mut cpu.memory;
    // add register 1 to register 0
    mem[0] = 0x80;
    mem[1] = 0x14;
    // add register 2 to register 0
    mem[2] = 0x80;
    mem[3] = 0x24;
    // add register 3 to register 0
    mem[4] = 0x80;
    mem[5] = 0x34;

    // function definition in memory.
    let add_twice: [u16; 3] = [0x2100, 0x2100, 0x0000];
    // decompose the [u16;3] array further into a [u8;6] array for CPU to read ops
    let add_twice: [u8; 6] = [0x21, 0x00, 0x21, 0x00, 0x00, 0x00];
    // load the function to RAM;
    mem[0x000..0x006].copy_from_slice(&add_twice);

    // // add ops
    let ret: [u16; 3] = [0x8014, 0x8014, 0x00EE];
    // decompose the [u16;3] array further into a [u8;6] array for CPU to read ops
    let ret: [u8; 6] = [0x80, 0x14, 0x80, 0x14, 0x00, 0xEE];
    // load the function to RAM;
    mem[0x100..0x106].copy_from_slice(&ret);

    println!(
        "registers are : {:?}, \nmemmory is : {:?}",
        cpu.registers, mem
    );

    cpu.run();
    println!(
        "registers:\n{:?}\nstack:\n{:?}\nmemory:\n{:?}",
        &cpu.registers,
        &cpu.stack,
        &cpu.memory[0..300]
    );
    assert_eq!(cpu.registers[0], 85);
}
