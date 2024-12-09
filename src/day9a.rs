use std::fs;

pub fn main() {
    let input = fs::read_to_string("input/9.txt").expect("Unable to read input file");

    let blocks = input.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

    let (files, spaces): (Vec<_>, Vec<_>) = blocks
        .iter()
        .enumerate()
        .partition(|(i, _)| i % 2 == 0);

    let mut files: Vec<u32> = files.iter().map(|(_, &b)| b).collect();
    let spaces: Vec<u32> = spaces.iter().map(|(_, &b)| b).collect();

    let mut left_file_index = 0;
    let mut right_file_index = files.len() - 1;
    let mut space_index = 0;

    let mut ans: u64 = 0;
    let mut is_files = true;
    let mut index = 0;

    while left_file_index <= right_file_index {
        if is_files {
            let file_blocks = files[left_file_index];

            ans += (0..file_blocks).map(|_| {
                let contribution = index * left_file_index as u64;
                index += 1;
                contribution
            }).sum::<u64>();

            files[left_file_index] = 0;
            is_files = false;
            left_file_index += 1;
        } else {
            if space_index >= spaces.len() {
                break;
            }

            let space_blocks = spaces[space_index];

            for _i in 0..space_blocks {
                while right_file_index >= left_file_index && files[right_file_index] == 0 {
                    if right_file_index == 0 {
                        dbg!(ans);
                        return;
                    }

                    right_file_index -= 1;
                }

                if right_file_index < left_file_index {
                    dbg!(ans);
                    return;
                }

                ans += index * right_file_index as u64;
                files[right_file_index] -= 1;
                index += 1;
            }

            is_files = true;
            space_index += 1;
        }
    }

    dbg!(ans);
}
