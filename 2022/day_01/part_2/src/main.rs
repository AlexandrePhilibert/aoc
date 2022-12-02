fn main() {
    let mut calories = include_str!("../input.txt")
        .lines()
        .fold(vec![0], |mut acc: Vec<u32>, line| {
            match line.parse::<u32>() {
                Ok(calories) => *acc.last_mut().unwrap() = *acc.last().unwrap() + calories,
                Err(_) => acc.push(0),
            }
            return acc;
        });

    calories.sort_by(|a, b| b.partial_cmp(a).unwrap());

    println!("{}", calories.iter().take(3).sum::<u32>());
}
