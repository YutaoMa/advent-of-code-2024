use std::fs;

#[derive(Debug)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug, Clone, Copy)]
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

impl From<u64> for Instruction {
    fn from(value: u64) -> Self {
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

fn combo(operand: u64, register: &Register) -> u64 {
    match operand {
        0..=3 => operand,
        4 => register.a,
        5 => register.b,
        6 => register.c,
        _ => panic!("Invalid operand"),
    }
}

fn eval(mut register: Register, program: &[u64]) -> Vec<u64> {
    let mut output = Vec::new();
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let instruction = Instruction::from(program[instruction_pointer]);
        let operand = program[instruction_pointer + 1];

        match instruction {
            Instruction::Adv => {
                let numerator = register.a;
                let denominator = u64::pow(2, combo(operand, &register).try_into().unwrap());
                register.a = numerator / denominator;
            }
            Instruction::Bxl => {
                register.b ^= operand;
            }
            Instruction::Bst => {
                register.b = combo(operand, &register) % 8;
            }
            Instruction::Jnz => {
                if register.a != 0 {
                    instruction_pointer = operand as usize;
                    continue;
                }
            }
            Instruction::Bxc => {
                register.b ^= register.c;
            }
            Instruction::Out => {
                output.push(combo(operand, &register) % 8);
            }
            Instruction::Bdv => {
                let numerator = register.a;
                let denominator = u64::pow(2, combo(operand, &register).try_into().unwrap());
                register.b = numerator / denominator;
            }
            Instruction::Cdv => {
                let numerator = register.a;
                let denominator = u64::pow(2, combo(operand, &register).try_into().unwrap());
                register.c = numerator / denominator;
            }
        }
        instruction_pointer += 2;
    }

    output
}

fn find(a: u64, b: u64, c: u64, program: &[u64], i: usize) {
    let initial_register = Register { a, b, c };
    let eval_result = eval(initial_register, program);

    if eval_result == program.iter().cloned().collect::<Vec<u64>>() {
        println!("{}", a);
    }

    if eval_result == program[program.len().saturating_sub(i)..] || i == 0 {
        for n in 0..8 {
            find(8 * a + n, b, c, program, i + 1);
        }
    }
}

fn parse_input(input: &str) -> (Register, Vec<u64>) {
    let mut lines = input.lines();
    let a = lines.next().unwrap().strip_prefix("Register A: ").unwrap().parse().unwrap();
    let b = lines.next().unwrap().strip_prefix("Register B: ").unwrap().parse().unwrap();
    let c = lines.next().unwrap().strip_prefix("Register C: ").unwrap().parse().unwrap();
    let program = lines.skip(1).next().unwrap().strip_prefix("Program: ").unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    (Register { a, b, c }, program)
}

pub fn main() {
    let input = fs::read_to_string("input/17.txt").expect("Unable to read input file");

    let (Register { a, b, c }, program) = parse_input(&input);

    let result = eval(Register { a, b, c }, program.as_slice());
    println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    find(0, b, c, program.as_slice(), 0);
}
