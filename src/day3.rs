use std::{io::{BufReader, BufRead}, fs::OpenOptions, collections::{HashSet, HashMap}};

use itertools::{Itertools, chain};

fn main() {
    let score_table = chain('a'..='z',  'A'..='Z')
        .enumerate()
        .map(|(idx, c)| (c, idx + 1))
        .collect::<HashMap<char, usize>>();        
    let reader = BufReader::new(OpenOptions::new().read(true).open("task_data/3.txt").unwrap());
    let lines = reader.lines()
        .map(|result| result.unwrap())
        .collect::<Vec<String>>();

    let line_score_sum = lines.iter()
        .map(|line| line.split_at(line.len()/2))
        .map(|(left, right)| left.chars()
            .filter(|&c| right.contains(c))
            .unique())
        .flatten()
        .map(|c| score_table[&c])
        .sum::<usize>();
    
    let group_score_sum = lines.chunks(3)
        .map(|chunk| chunk.iter()
            .map(|s| s.chars().collect::<HashSet<char>>())
            .reduce(|left, right| left.intersection(&right).cloned().collect()))
        .flatten().flatten()
        .map(|c| score_table[&c])
        .sum::<usize>();
    
    println!("Lines score: {line_score_sum}, groups score: {group_score_sum}");
}