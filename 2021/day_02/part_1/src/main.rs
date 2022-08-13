fn main() {
    let (horizontal_pos, depth) = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
        .fold((0, 0), |(horizontal_pos, depth), (dir, val)| {
            match (dir, val.parse::<usize>().unwrap()) {
                ("forward", val) => (horizontal_pos + val, depth),
                ("up", val) => (horizontal_pos, depth - val),
                ("down", val) => (horizontal_pos, depth + val),
                _ => (horizontal_pos, depth),
            }
        });

    println!("{}", horizontal_pos * depth)
}
