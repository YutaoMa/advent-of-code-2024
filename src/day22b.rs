use std::collections::HashMap;
use std::fs;

struct Secret(i64);

impl Secret {
    fn mix(&mut self, other: i64) {
        self.0 ^= other;
    }

    fn prune(&mut self) {
        self.0 %= 16_777_216;
    }

    fn next(&mut self) {
        const FACTORS: [(i64, i64); 3] = [(64, 1), (32, -1), (2048, 1)];

        for &(factor, sign) in &FACTORS {
            let value = if sign == 1 { self.0 * factor } else { self.0 / factor };
            self.mix(value);
            self.prune();
        }
    }
}

fn generate_steps(numbers: &[i64], steps: usize) -> Vec<Vec<i64>> {
    let mut steps_array: Vec<Vec<i64>> = vec![vec![0; steps]; numbers.len()];

    for (i, &num) in numbers.iter().enumerate() {
        let mut secret = Secret(num);
        for step in 0..steps {
            secret.next();
            steps_array[i][step] = secret.0;
        }
    }

    steps_array
}

fn max_bananas(numbers: &[i64]) -> i64 {
    let steps = 2000;
    let steps_array = generate_steps(numbers, steps);

    let prices: Vec<Vec<i64>> = steps_array
        .iter()
        .map(|row| row.iter().map(|&x| x % 10).collect())
        .collect();

    let diffs: Vec<Vec<i64>> = prices
        .iter()
        .map(|row| row.windows(2).map(|w| w[1] - w[0]).collect())
        .collect();

    let num_sequences = steps - 4;

    let mut sequence_map: HashMap<i64, i64> = HashMap::new();

    for (number_id, diff_row) in diffs.iter().enumerate() {
        for seq_start in 0..num_sequences {
            let seq: Vec<i64> = diff_row[seq_start..seq_start + 4]
                .iter()
                .map(|&d| d + 10)
                .collect();

            let sequence_id = seq[0] * 19_i64.pow(3)
                + seq[1] * 19i64.pow(2)
                + seq[2] * 19
                + seq[3];

            let key = (number_id as i64) * 19_i64.pow(4) + sequence_id;
            let banana_count = prices[number_id][seq_start + 4];

            sequence_map.entry(key).or_insert(banana_count);
        }
    }

    let mut sum_banana: Vec<i64> = vec![0; 19_usize.pow(4)];

    for (&key, &banana_count) in &sequence_map {
        let sequence_id = key % 19i64.pow(4);
        sum_banana[sequence_id as usize] += banana_count;
    }

    *sum_banana.iter().max().unwrap_or(&0)
}

pub fn main() {
    let input = fs::read_to_string("input/22.txt").expect("Unable to read input file");

    let numbers: Vec<i64> = input
        .lines()
        .map(|line| line.parse::<i64>().expect("Invalid input number"))
        .collect();

    let result = max_bananas(&numbers);
    println!("Part2 result: {}", result);
}
