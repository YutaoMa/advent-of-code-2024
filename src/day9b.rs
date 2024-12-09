use std::fs;

#[derive(Debug)]
enum Block {
    File(u32, u32),
    Space(u32),
}

pub fn main() {
    let input = fs::read_to_string("input/9.txt").expect("Unable to read input file");

    let mut blocks = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, b)| {
            if i % 2 == 0 {
                Block::File(b, (i / 2) as u32)
            } else {
                Block::Space(b)
            }
        })
        .collect::<Vec<Block>>();

    let mut i = blocks.len() - 1;

    while i > 0 {
        if let Block::File(b, file_id) = blocks[i] {
            for j in 0..i {
                if let Block::Space(s) = blocks[j] {
                    if s >= b {
                        blocks[j] = Block::File(b, file_id);
                        blocks[i] = Block::Space(b);
                        if s > b {
                            blocks.insert(j + 1, Block::Space(s - b));
                            i += 1;
                        }
                        break;
                    }
                }
            }
        }

        i -= 1;
    }

    let mut index = 0;
    let mut ans: u64 = 0;
    for block in blocks.iter() {
        match block {
            Block::File(b, file_id) => {
                ans += (*file_id as u64) * (index..index+*b as usize).sum::<usize>() as u64;
                index += *b as usize;
            },
            Block::Space(b) => {
                index += *b as usize;
            },
        }
    }

    dbg!(ans);
}
