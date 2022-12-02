fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .fold(vec![0], |mut acc: Vec<u32>, line| {
                match line.parse::<u32>() {
                    Ok(calories) => *acc.last_mut().unwrap() = *acc.last().unwrap() + calories,
                    Err(_) => acc.push(0),
                }

                return acc;
            })
            .iter()
            .max()
            .unwrap()
        );
}
