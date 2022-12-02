use itertools::Itertools;

#[derive(Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum GameResult {
    Won = 6,
    Lost = 0,
    Draw = 3,
}

fn decode_shape(shape: &str) -> Shape {
    match shape {
        "A" | "X" => Shape::Rock,
        "B" | "Y" => Shape::Paper,
        "C" | "Z" => Shape::Scissors,
        _ => panic!("Error in input"),
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
            .map(|(a, b)| (decode_shape(a), decode_shape(b)))
            .fold(0, |acc, (a, b)| acc + get_result(a, b) as u32 + b as u32)
    );
}
