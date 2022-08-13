fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            .windows(2)
            .filter(|x| x[1] > x[0])
            .count()
    );
}
