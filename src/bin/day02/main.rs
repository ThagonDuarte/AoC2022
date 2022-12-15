use std::fs;

enum CHOICES {
    None,
    Rock,
    Paper,
    Scissors,
}

enum OUTCOME {
    None,
    Lose,
    Draw,
    Win,
}

fn main() {
    let filepath = "src/bin/day02/input.txt";
    let input = fs::read_to_string(filepath).expect("Should have been able to read the file.");
    let mut score_1: u32 = 0;
    let mut score_2: u32 = 0;

    for elem in input.lines() {
        score_1 += score_of_round(parse_round_1(elem));
        score_2 += score_of_round(parse_round_2(elem));
    }
    println!("The resulting score from task 1 is {}", score_1);
    println!("The resulting score from task 1 is {}", score_2);
}

fn score_of_round(round: (CHOICES, CHOICES)) -> u32 {
    let mut score: u32 = 0;

    match round {
        (CHOICES::None, _) => score += 0,
        (_, CHOICES::None) => score += 0,
        (CHOICES::Rock, CHOICES::Scissors) => score += 0,
        (CHOICES::Paper, CHOICES::Rock) => score += 0,
        (CHOICES::Scissors, CHOICES::Paper) => score += 0,
        (CHOICES::Rock, CHOICES::Paper) => score += 6,
        (CHOICES::Paper, CHOICES::Scissors) => score += 6,
        (CHOICES::Scissors, CHOICES::Rock) => score += 6,
        (_, _) => score += 3,
    }

    match round.1 {
        CHOICES::None => score += 0,
        CHOICES::Rock => score += 1,
        CHOICES::Paper => score += 2,
        CHOICES::Scissors => score += 3,
    }
    return score;
}

fn parse_round_1(elem: &str) -> (CHOICES, CHOICES) {
    let mut choices = elem.split_whitespace();
    let mut round: (CHOICES, CHOICES) = (CHOICES::None, CHOICES::None);
    match choices.next().unwrap() {
        "A" => round.0 = CHOICES::Rock,
        "B" => round.0 = CHOICES::Paper,
        "C" => round.0 = CHOICES::Scissors,
        _ => round.0 = CHOICES::None,
    }
    match choices.next().unwrap() {
        "X" => round.1 = CHOICES::Rock,
        "Y" => round.1 = CHOICES::Paper,
        "Z" => round.1 = CHOICES::Scissors,
        _ => round.1 = CHOICES::None,
    }

    return round;
}

fn parse_round_2(elem: &str) -> (CHOICES, CHOICES) {
    let mut choices = elem.split_whitespace();
    let mut round: (CHOICES, OUTCOME) = (CHOICES::None, OUTCOME::None);
    match choices.next().unwrap() {
        "A" => round.0 = CHOICES::Rock,
        "B" => round.0 = CHOICES::Paper,
        "C" => round.0 = CHOICES::Scissors,
        _ => round.0 = CHOICES::None,
    }
    match choices.next().unwrap() {
        "X" => round.1 = OUTCOME::Lose,
        "Y" => round.1 = OUTCOME::Draw,
        "Z" => round.1 = OUTCOME::Win,
        _ => round.1 = OUTCOME::None,
    }

    let mut result: (CHOICES, CHOICES) = (CHOICES::None, CHOICES::None);
    match round {
        (CHOICES::Rock, OUTCOME::Lose) => result = (CHOICES::Rock, CHOICES::Scissors),
        (CHOICES::Rock, OUTCOME::Draw) => result = (CHOICES::Rock, CHOICES::Rock),
        (CHOICES::Rock, OUTCOME::Win) => result = (CHOICES::Rock, CHOICES::Paper),
        (CHOICES::Paper, OUTCOME::Lose) => result = (CHOICES::Paper, CHOICES::Rock),
        (CHOICES::Paper, OUTCOME::Draw) => result = (CHOICES::Paper, CHOICES::Paper),
        (CHOICES::Paper, OUTCOME::Win) => result = (CHOICES::Paper, CHOICES::Scissors),
        (CHOICES::Scissors, OUTCOME::Lose) => result = (CHOICES::Scissors, CHOICES::Paper),
        (CHOICES::Scissors, OUTCOME::Draw) => result = (CHOICES::Scissors, CHOICES::Scissors),
        (CHOICES::Scissors, OUTCOME::Win) => result = (CHOICES::Scissors, CHOICES::Rock),
        (_, _) => result = (CHOICES::None, CHOICES::None),
    }

    return result;
}
