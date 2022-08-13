fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .fold(0, |acc, mass| acc + (mass / 3) - 2)
    );
}
