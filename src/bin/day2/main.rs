use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

fn checksum() -> u64 {
    let (mut twos, mut threes) = (0, 0);

    for line in INPUT.lines() {
        let mut char_counts = HashMap::new();
        let (mut line_twos, mut line_threes) = (0, 0);
        for ch in line.chars() {
            *char_counts.entry(ch).or_insert(0) += 1;
        }
        for &count in char_counts.values() {
            if count == 2 {
                line_twos += 1;
            }
            if count == 3 {
                line_threes += 1;
            }
        }
        twos += (line_twos > 0) as u64;
        threes += (line_threes > 0) as u64;
    }

    twos * threes
}

fn common_letters_sibling_box_ids() -> String {
    let parsed_input: Vec<&str> = INPUT.lines().collect();

    for i in 0..(parsed_input.len()) {
        for j in (i + 1)..(parsed_input.len()) {
            let mut differing = 0;
            let mut common = vec![];

            for (l, r) in parsed_input[i].chars().zip(parsed_input[j].chars()) {
                if l != r {
                    differing += 1;
                } else {
                    common.push(l);
                }
            }
            if differing == 1 {
                return common.iter().collect();
            }
        }
    }

    unreachable!();
}

fn main() {
    println!("part 1: {}", checksum());
    println!("part 2: {}", common_letters_sibling_box_ids());
}