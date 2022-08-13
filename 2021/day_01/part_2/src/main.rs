fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .windows(4)
            .filter(|x| x[0] < x[3])
            .count()
    );
}
