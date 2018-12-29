use std::collections::HashSet;
use lazy_static::lazy_static;

const INPUT: &str = include_str!("./input.txt");

lazy_static! {
    static ref PARSED_INPUT: Vec<i64> = INPUT
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();
    static ref SUMS: Vec<i64> = PARSED_INPUT
        .iter()
        .scan(0, |acc, change| {
            *acc += change;
            Some(*acc)
        })
        .collect();
}

fn sum_of_frequency_changes() -> i64 {
    *SUMS.last().unwrap()
}

fn first_freq_reached_twice() -> i64 {
    let freqs_reached: HashSet<i64> = SUMS.iter().cloned().collect();
    let mut current_freq = sum_of_frequency_changes();
    
    for change in PARSED_INPUT.iter().cycle() {
        current_freq += change;
        if freqs_reached.contains(&current_freq) {
            return current_freq;
        }
    }

    unreachable!();
}

fn main() {
    println!("part 1: {}", sum_of_frequency_changes());
    println!("part 2: {}", first_freq_reached_twice());
}