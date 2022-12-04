use std::{io::{BufReader, BufRead}, fs::OpenOptions};

use regex::Regex;

fn main() {
    let reader = BufReader::new(OpenOptions::new().read(true).open("task_data/4.txt").unwrap());
    let lines = reader.lines()
        .flatten()
        .collect::<Vec<String>>();
    
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let range_pairs = lines.iter()
        .map(|line| regex.captures(line).unwrap())
        .map(|cap| (
            cap[1].parse::<u32>().unwrap()..=cap[2].parse::<u32>().unwrap(),
            cap[3].parse::<u32>().unwrap()..=cap[4].parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    
    let fully_contained_ranges_count = range_pairs.iter()
        .filter(|(left, right)|
            (left.start() >= right.start() && left.end() <= right.end()) ||
            (left.start() <= right.start() && left.end() >= right.end()))
        .count();
    
    let overlapping_ranges_count = range_pairs.iter()
        .filter(|(left, right)|
            left.start() <= right.end() && left.end() >= right.start())
        .count();
    
    println!("Fully contained ranges count: {fully_contained_ranges_count}");
    println!("Overlapping ranges count: {overlapping_ranges_count}");
}