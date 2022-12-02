use std::{fs::OpenOptions, io::{BufReader, BufRead}};

fn main() -> anyhow::Result<()> {
    let file = OpenOptions::new()
        .read(true)
        .open("task_data/1.txt")?;
    let reader = BufReader::new(file);

    let mut sums = vec![];
    let mut current_sum = 0;

    for line in reader.lines() {
        match (line?).as_str() {
            "" => {
                sums.push(current_sum);
                current_sum = 0;    
            }
            line => {
                let value = line.parse::<u64>()?;
                current_sum += value;
            }
        }
    }
    sums.push(current_sum);

    sums.sort();
    sums.reverse();
    println!("Top sum: {}", sums.get(0).unwrap_or(&0));
    println!("Top 3 sum: {}", sums.iter().take(3).sum::<u64>());
    Ok(())
}
