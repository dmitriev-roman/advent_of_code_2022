use std::{fs::OpenOptions, io::{BufReader, BufRead}};

use anyhow::anyhow;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn beats(&self) -> Self {
        match self {
            Shape::Rock => Self::Scissors,
            Shape::Paper => Self::Rock,
            Shape::Scissors => Self::Paper,
        }
    }

    fn loses(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = anyhow::Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(anyhow!("{value} is not a valid shape"))
        }
    }
}

#[derive(Debug)]
enum RoundResult {
    Lose,
    Draw,
    Win
}

impl TryFrom<char> for RoundResult {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(RoundResult::Lose),
            'Y' => Ok(RoundResult::Draw),
            'Z' => Ok(RoundResult::Win),
            _ => Err(anyhow!("{value} is not a valid round result"))
        }
    }
}

fn play_round(yours: &Shape, opponent: &Shape) -> RoundResult {
    if *yours == *opponent {
        RoundResult::Draw
    } else if yours.beats() == *opponent {
        RoundResult::Win
    } else {
        RoundResult::Lose
    }
}

fn match_shape(opponent: &Shape, expected_result: &RoundResult) -> Shape {
    match *expected_result {
        RoundResult::Lose => opponent.beats(),
        RoundResult::Draw => *opponent,
        RoundResult::Win => opponent.loses(),
    }
}

fn shape_score(shape: &Shape) -> u64 {
    match *shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn result_score(result: &RoundResult) -> u64 {
    match *result {
        RoundResult::Lose => 0,
        RoundResult::Draw => 3,
        RoundResult::Win => 6,
    }
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(OpenOptions::new().read(true).open("task_data/2.txt")?);
    let mut first_rule_set_total_score = 0;
    let mut second_rule_set_total_score = 0;
    for line in reader.lines() {
        let chars: Vec<char> = line?.chars().collect();
        let opponent_hand = chars[0].try_into()?;
        let first_rule_set_your_hand = chars[2].try_into()?;
        let first_rule_set_result = play_round(&first_rule_set_your_hand, &opponent_hand);
        let first_rule_set_score = shape_score(&first_rule_set_your_hand) + result_score(&first_rule_set_result);
        first_rule_set_total_score += first_rule_set_score;

        let expected_result = chars[2].try_into()?;
        let excpected_hand = match_shape(&opponent_hand, &expected_result);
        let second_rule_set_score = shape_score(&excpected_hand) + result_score(&expected_result);
        second_rule_set_total_score += second_rule_set_score;

        println!("1st rule set: {:?} vs {:?}: {:?} ({})", first_rule_set_your_hand, opponent_hand, first_rule_set_result, first_rule_set_score);
        println!("2nd rule set: {:?} vs {:?}: {:?} ({})", excpected_hand, opponent_hand, expected_result, second_rule_set_score);
    }
    println!("First rule set score: {first_rule_set_total_score}");
    println!("Second rule set score: {second_rule_set_total_score}");
    Ok(())
}