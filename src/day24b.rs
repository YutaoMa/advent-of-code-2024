use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug)]
struct Instruction {
    op: String,
    arg1: String,
    arg2: String,
    dst: String,
}

fn parse_instructions(section: &str) -> Vec<Instruction> {
    section
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            Instruction {
                arg1: parts.next().unwrap().to_string(),
                op: parts.next().unwrap().to_string(),
                arg2: parts.next().unwrap().to_string(),
                dst: parts.skip(1).next().unwrap().to_string(), // skip the '->' token
            }
        })
        .collect()
}

fn build_outputs(instructions: &[Instruction]) -> HashMap<String, HashSet<String>> {
    let mut outputs = HashMap::new();
    for instruction in instructions {
        outputs
            .entry(instruction.arg1.clone())
            .or_insert_with(HashSet::new)
            .insert(instruction.op.clone());
        outputs
            .entry(instruction.arg2.clone())
            .or_insert_with(HashSet::new)
            .insert(instruction.op.clone());
    }
    outputs
}

fn process_instructions(
    instructions: &[Instruction],
    outputs: &HashMap<String, HashSet<String>>,
) -> HashSet<String> {
    instructions
        .iter()
        .filter_map(|instruction| match instruction.op.as_str() {
            "AND" if instruction.arg1 != "x00"
                && instruction.arg2 != "x00"
                && outputs.get(&instruction.dst).map_or(true, |s| !s.contains("OR")) =>
            {
                Some(instruction.dst.clone())
            }
            "OR" if instruction.dst.starts_with('z')
                && instruction.dst != "z45"
                || outputs
                    .get(&instruction.dst)
                    .map_or(false, |s| s.contains("OR")) =>
            {
                Some(instruction.dst.clone())
            }
            "XOR" if instruction.arg1.starts_with('x') || instruction.arg2.starts_with('x') => {
                if instruction.arg1 != "x00"
                    && instruction.arg2 != "x00"
                    && outputs.get(&instruction.dst).map_or(true, |s| !s.contains("XOR"))
                {
                    Some(instruction.dst.clone())
                } else {
                    None
                }
            }
            "XOR" if !instruction.dst.starts_with('z') => Some(instruction.dst.clone()),
            _ => None,
        })
        .collect()
}

pub fn main() {
    let input = fs::read_to_string("input/24.txt").expect("Unable to read input file");

    let sections = input.split("\n\n");
    let instructions_section = sections.skip(1).next().expect("Missing instruction section");

    let instructions = parse_instructions(instructions_section);
    let outputs = build_outputs(&instructions);
    let mut ans: Vec<String> = process_instructions(&instructions, &outputs)
        .into_iter()
        .collect();

    ans.sort();
    println!("{}", ans.join(","));
}
