use std::iter::successors;

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .fold(0, |acc, mass| acc
                + successors(Some((mass / 3).saturating_sub(2)), |m| {
                    Some((m / 3).saturating_sub(2))
                })
                .take_while(|&m| m != 0)
                .sum::<usize>())
    );
}
