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

pub fn main() {
    let input = fs::read_to_string("input/22.txt").expect("Unable to read input file");

    let mut secrets: Vec<Secret> = input
        .lines()
        .map(|line| line.parse::<i64>().expect("Invalid input number"))
        .map(Secret)
        .collect();

    for _ in 0..2000 {
        secrets.iter_mut().for_each(|secret| secret.next());
    }

    let ans: i64 = secrets.iter().map(|secret| secret.0).sum();
    println!("{}", ans);
}
