#![feature(try_trait)]

use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &str = include_str!("./input.txt");

struct Entry {
    claim: String,
    x: u64,
    y: u64,
    w: u64,
    h: u64,
}

enum ParseEntryError {
    NoneError(std::option::NoneError),
    ParseIntError(std::num::ParseIntError),
}

impl From<std::option::NoneError> for ParseEntryError {
    fn from(o: std::option::NoneError) -> Self {
        ParseEntryError::NoneError(o)
    }
}

impl From<std::num::ParseIntError> for ParseEntryError {
    fn from(o: std::num::ParseIntError) -> Self {
        ParseEntryError::ParseIntError(o)
    }
}

impl FromStr for Entry {
    type Err = ParseEntryError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        
        let claim = iter.next()?.to_owned();
        let _ = iter.next()?;
        let (x, y) = {
            let mut parts = iter.next()?.trim_matches(':').split(',');
            (parts.next()?.parse()?, parts.next()?.parse()?)
        };
        let (w, h) = {
            let mut parts = iter.next()?.split('x');
            (parts.next()?.parse()?, parts.next()?.parse()?)
        };
        
        Ok(Entry {
            claim,
            x,
            y,
            w,
            h,
        })
    }
}

fn rect_to_coords((x, y): (u64, u64), (w, h): (u64, u64)) -> Vec<(u64, u64)> {
    let mut coords = vec![];
    
    for y_offset in 0..h {
        for x_offset in 0..w {
            coords.push((x + x_offset, y + y_offset));
        }
    }
    
    coords
}

fn overlap_area(map: &HashSet<(u64, u64)>) -> u64 {
    map.len() as u64
}

fn main() {
    let mut total_coverage_map = HashSet::new();
    let mut intersection_map = HashSet::new();
    
    for entry in INPUT.lines() {
        let Entry { x, y, w, h, .. } = entry.parse().ok().unwrap();
        let coords = rect_to_coords((x, y), (w, h));
        for coord in &coords {
            if total_coverage_map.contains(coord) {
                intersection_map.insert(*coord);
            }
        }
        total_coverage_map.extend(coords.iter().cloned());
    }
    
    println!("part 1: {}", overlap_area(&intersection_map));
    
    for entry in INPUT.lines() {
        let Entry { claim, x, y, w, h, .. } = entry.parse().ok().unwrap();
        let coords = rect_to_coords((x, y), (w, h));
        if !coords.iter().any(|coord| intersection_map.contains(coord)) {
            println!("part 2: {}", claim);
        }
    }
}