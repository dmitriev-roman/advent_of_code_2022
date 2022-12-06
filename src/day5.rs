use std::{fs::File, io::{BufReader, BufRead}};

use regex::Regex;

fn read_dataset() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let lines = BufReader::new(File::open("task_data/5.txt").unwrap()).lines().flatten().collect::<Vec<_>>();
    let stacks_end_idx = lines.iter().position(|line| line.is_empty()).unwrap();
    let stacks = read_stacks(lines.iter().take(stacks_end_idx).rev().cloned().collect());
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let moves = lines.iter()
        .skip(stacks_end_idx+1)
        .map(|line| regex.captures(line).unwrap())
        .map(|cap| (cap[1].parse::<usize>().unwrap(), cap[2].parse::<usize>().unwrap()-1, cap[3].parse::<usize>().unwrap()-1))
        .collect::<Vec<_>>();
    (stacks, moves)
}

fn read_stacks(lines: Vec<String>) -> Vec<Vec<char>> {
    let stacks_count = lines.get(0).unwrap().chars().map(|x| x.to_digit(10)).flatten().max().unwrap() as usize;
    let mut stacks = vec![vec![]; stacks_count];
    for line in lines.iter().skip(1) {
        let chars = line.chars().collect::<Vec<_>>();
        for idx in 0..stacks_count {
            let c = chars[1 + idx*4];
            if c != ' ' {
                stacks[idx].push(c);
            }
        }
    }
    stacks
}


fn main() {
    let (mut stacks, moves) = read_dataset();
    let mut stacks_v2 = stacks.clone();
    for (count, from, to) in moves {
        let mut buffer = vec![];
        for _ in 0..count {
            let c = stacks.get_mut(from).unwrap().pop().unwrap();
            buffer.push(stacks_v2.get_mut(from).unwrap().pop().unwrap());
            stacks.get_mut(to).unwrap().push(c);
        }
        stacks_v2.get_mut(to).unwrap().extend(buffer.iter().rev());
    }
    let result = stacks.iter()
        .map(|stack| stack.last())
        .flatten()
        .collect::<String>();
        let result_v2 = stacks_v2.iter()
        .map(|stack| stack.last())
        .flatten()
        .collect::<String>();
    println!("{result}");
    println!("{result_v2}");
}