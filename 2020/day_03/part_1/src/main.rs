fn main() {
    let map: Vec<Vec<bool>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    println!(
        "{}",
        (0..map.len() - 1)
            .map(|i| ((i + 1) * 3, i + 1))
            .filter(|(x, y)| map[*y][*x % map[0].len()])
            .count()
    )
}
