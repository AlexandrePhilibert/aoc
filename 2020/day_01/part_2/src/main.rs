use itertools::Itertools;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .combinations(3)
            .find(|c| c.iter().sum::<usize>() == 2020)
            .unwrap()
            .iter()
            .product::<usize>()
    );
}
