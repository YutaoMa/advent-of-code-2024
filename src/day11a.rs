use std::fs;

fn transition(num: i64) -> Vec<i64> {
    let mut new_num: Vec<i64> = Vec::new();

    if num == 0 {
        new_num.push(1);
    } else {
        let num_str = num.to_string();
        if num_str.len() % 2 == 0 {
            let num_str_first_half = &num_str[..num_str.len()/2];
            let num_str_second_half = &num_str[num_str.len()/2..];

            let first_half = num_str_first_half.parse::<i64>().unwrap();
            let second_half = num_str_second_half.parse::<i64>().unwrap();

            new_num.push(first_half);
            new_num.push(second_half);
        } else {
            new_num.push(num * 2024);
        }
    }

    new_num
}

pub fn main() {
    let input = fs::read_to_string("input/11.txt").expect("Unable to read input file");

    let mut nums = input.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    for i in 0..25 {
        dbg!(i);

        let mut new_nums: Vec<i64> = Vec::new();

        for num in nums.iter() {
            let new_num = transition(*num);
            new_nums.extend(new_num);
        }

        nums = new_nums.clone();
    }

    dbg!(nums.len());
}
