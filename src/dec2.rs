#[allow(dead_code)]
const NIY : &str = "Not implemented yet";

#[derive(Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn str_to_shape1(s: &str) -> Shape {
    match s {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,

        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissors,

        _ => panic!("Unexpected input {}", s),
    }
}

fn str_to_shape2(s: &str) -> Shape {
    match s {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,

        _ => panic!("Unexpected input {}", s),
    }
}

fn str_to_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,

        _ => panic!("Unexpected input {}", s),
    }
}

fn play_round1(op_choice: &Shape, my_choice: &Shape) -> Outcome {
    match (op_choice, my_choice) {
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,
        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Paper, Shape::Rock) => Outcome::Lose,
        (Shape::Scissors, Shape::Paper) => Outcome::Lose,
        (Shape::Rock, Shape::Scissors) => Outcome::Lose,
        _ => Outcome::Draw,
    }
}

fn play_round2(op_choice: &Shape, outcome: &Outcome) -> Shape {
    match (op_choice, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (shape, _) => shape.clone(),
    }
}

fn shape_to_point(shape: Shape) ->i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn outcome_to_point(outcome: Outcome) ->i32 {
    match outcome {
        Outcome::Lose => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

#[allow(unused_variables)]
pub fn solution1(filename: &str) -> Result<i32, &str> {
    let s = std::fs::read_to_string(filename).unwrap();
    
    let mut total_points = 0;
    for line in s.lines() {
        let round = line.split_whitespace().collect::<Vec<&str>>();
        let op_choice = str_to_shape1(round[0]);
        let my_choice = str_to_shape1(round[1]);
        let outcome = play_round1(&op_choice, &my_choice);
        let point = shape_to_point(my_choice) + outcome_to_point(outcome);
        total_points += point;
    }

    Ok(total_points)
}

#[allow(unused_variables)]
pub fn solution2(filename: &str) -> Result<i32, &str> {
    let s = std::fs::read_to_string(filename).unwrap();
    
    let mut total_points = 0;
    for line in s.lines() {
        let round = line.split_whitespace().collect::<Vec<&str>>();
        let op_choice = str_to_shape2(round[0]);
        let outcome = str_to_outcome(round[1]);
        let my_choice = play_round2(&op_choice, &outcome);
        let point = shape_to_point(my_choice) + outcome_to_point(outcome);
        total_points += point;
    }

    // Err(NIY)
    Ok(total_points)
}
