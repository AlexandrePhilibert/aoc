use itertools::Itertools;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
enum GameResult {
    Won = 6,
    Lost = 0,
    Draw = 3,
}

fn decode_shape(shape: &str) -> Shape {
    match shape {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissors,
        _ => panic!("Error in input"),
    }
}

fn decode_result(shape: &str) -> GameResult {
    match shape {
        "X" => GameResult::Lost,
        "Y" => GameResult::Draw,
        "Z" => GameResult::Won,
        _ => panic!("Error in input"),
    }
}

fn shape_for_game_result(shape: Shape, result: GameResult) -> Shape {
    match (shape, result) {
        (Shape::Rock, GameResult::Won)
        | (Shape::Paper, GameResult::Draw)
        | (Shape::Scissors, GameResult::Lost) => Shape::Paper,
        (Shape::Rock, GameResult::Lost)
        | (Shape::Paper, GameResult::Won)
        | (Shape::Scissors, GameResult::Draw) => Shape::Scissors,
        (Shape::Rock, GameResult::Draw)
        | (Shape::Paper, GameResult::Lost)
        | (Shape::Scissors, GameResult::Won) => Shape::Rock,
    }
}

fn get_result(a: Shape, b: Shape) -> GameResult {
    match (a, b) {
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => GameResult::Lost,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => GameResult::Won,
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => GameResult::Draw,
    }
}

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|game| game.split(' ').collect_tuple().unwrap())
            .map(|(a, b)| (decode_shape(a), decode_result(b)))
            .map(|(a, b)| (a, shape_for_game_result(a, b)))
            .fold(0, |acc, (a, b)| acc + get_result(a, b) as u32 + b as u32)
    );
}
