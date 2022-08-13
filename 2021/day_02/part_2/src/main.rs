fn main() {
    let (horizontal_pos, depth, _) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold(
            (0, 0, 0),
            |(horizontal_pos, depth, aim), (dir, val)| match (dir, val.parse::<usize>().unwrap()) {
                ("forward", val) => (horizontal_pos + val, depth + val * aim, aim),
                ("up", val) => (horizontal_pos, depth, aim - val),
                ("down", val) => (horizontal_pos, depth, aim + val),
                _ => (horizontal_pos, depth, aim),
            },
        );

    println!("{}", horizontal_pos * depth)
}
