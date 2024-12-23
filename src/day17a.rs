use std::fs;

#[derive(Debug)]
struct Register {
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u32> for Instruction {
    fn from(value: u32) -> Self {
        match value {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => panic!("Invalid instruction"),
        }
    }
}

fn combo(operand: u32, register: &Register) -> u32 {
    match operand {
        0..=3 => operand,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Invalid operand"),
    }
}

fn eval(instruction: Instruction, operand: u32, register: &mut Register, instruction_pointer: &mut usize, output: &mut Vec<u32>) {
    match instruction {
        Instruction::Adv => {
            let numerator = register.a;
            let denominator = u32::pow(2, combo(operand, register));
            register.a = numerator / denominator;
        },
        Instruction::Bxl => {
            register.b ^= operand;
        },
        Instruction::Bst => {
            register.b = combo(operand, register) % 8;
        },
        Instruction::Jnz => {
            if register.a != 0 {
                *instruction_pointer = operand as usize;
                return;
            }
        },
        Instruction::Bxc => {
            register.b ^= register.c;
        },
        Instruction::Out => {
            output.push(combo(operand, register) % 8);
        },
        Instruction::Bdv => {
            let numerator = register.a;
            let denominator = u32::pow(2, combo(operand, register));
            register.b = numerator / denominator;
        },
        Instruction::Cdv => {
            let numerator = register.a;
            let denominator = u32::pow(2, combo(operand, register));
            register.c = numerator / denominator;
        },
    }
    *instruction_pointer += 2;
}

fn parse_input(input: &str) -> (Register, Vec<u32>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap().strip_prefix("Register A: ").unwrap().parse().unwrap();
    let b = lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse().unwrap();
    let c = lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse().unwrap();
    let program = lines.skip(1).next().unwrap().strip_prefix("Program: ").unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    (Register { a, b, c }, program)
}

pub fn main() {
    let input = fs::read_to_string("input/17.txt").expect("Unable to read input file");

    let (mut register, program) = parse_input(&input);

    let mut output = Vec::new();

    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let instruction = Instruction::from(program[instruction_pointer]);
        let operand = program[instruction_pointer + 1];
        eval(instruction, operand, &mut register, &mut instruction_pointer, &mut output);
    }

    println!("{}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));
}
