use std::{fs::File, io::Read, slice::Windows};

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    File::open("task_data/6.txt").unwrap().read_to_string(&mut input).unwrap();
    let chars = input.chars().collect::<Vec<_>>();
    let (first_marker_idx, first_marker) = chars 
        .windows(4).enumerate()
        .filter(|(_, window)| window.iter().all_unique() )
        .next().unwrap();
    let (second_marker_idx, second_marker) = chars 
        .windows(14).enumerate()
        .filter(|(_, window)| window.iter().all_unique() )
        .next().unwrap();
    println!("First marker at {}: {}, (ends at {})", first_marker_idx, first_marker.iter().collect::<String>(), first_marker_idx + first_marker.len());
    println!("Second marker at {}: {}, (ends at {})", second_marker_idx, second_marker.iter().collect::<String>(), second_marker_idx + second_marker.len());
}