use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Instruction {
    op: String,
    arg1: String,
    arg2: String,
    dst: String,
}

pub fn main() {
    let input = fs::read_to_string("input/24.txt").expect("Unable to read input file");

    let mut sections = input.split("\n\n");

    let mut registers: HashMap<String, bool> = sections
        .next()
        .expect("Missing initial state section")
        .lines()
        .map(|line| {
            let (key, value) = line.split_once(": ").expect("Invalid register format");
            (key.to_string(), value == "1")
        })
        .collect();

    let mut instructions: Vec<Instruction> = sections
        .next()
        .expect("Missing instructions section")
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Instruction {
                arg1: parts.next().expect("Missing arg1").to_string(),
                op: parts.next().expect("Missing op").to_string(),
                arg2: parts.next().expect("Missing arg2").to_string(),
                dst: parts.nth(1).expect("Missing dst").to_string(),
            }
        })
        .collect();

    while let Some(instruction) = instructions.pop() {
        let Some(&arg1) = registers.get(&instruction.arg1) else {
            instructions.insert(0, instruction);
            continue;
        };
        let Some(&arg2) = registers.get(&instruction.arg2) else {
            instructions.insert(0, instruction);
            continue;
        };

        let value = match instruction.op.as_str() {
            "AND" => arg1 & arg2,
            "OR" => arg1 | arg2,
            "XOR" => arg1 ^ arg2,
            _ => panic!("Unknown operation: {}", instruction.op),
        };

        registers.insert(instruction.dst, value);
    }

    let ans: String = (0..=45)
        .rev()
        .map(|i| {
            let key = format!("z{:02}", i);
            registers
                .get(&key)
                .map(|&v| if v { '1' } else { '0' })
                .unwrap_or('0')
        })
        .collect();

    dbg!(u64::from_str_radix(&ans, 2).expect("Invalid binary number"));
}
