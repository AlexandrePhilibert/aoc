fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .fold(vec![0], |mut acc: Vec<u32>, line| {
                if let Ok(calories) = line.parse::<u32>() {
                    *acc.last_mut().unwrap() = *acc.last().unwrap() + calories;
                } else {
                   acc.push(0);
                }

               return acc;
            })
            .iter()
            .max()
            .unwrap()
        );
}
