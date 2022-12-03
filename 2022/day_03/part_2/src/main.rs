use itertools::Itertools;

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 26 + 1,
        _ => unreachable!("How ?"),
    }
}

fn intersection(a: &str, b: &str, c: &str) -> char {
    return c
        .chars()
        .find(|char| {
            a.chars()
                .filter(|char| b.contains(*char))
                .find(|char1| char1 == char)
                .is_some()
        })
        .unwrap();
}

fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| String::from(l))
            .collect::<Vec<String>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect_tuple().unwrap())
            .map(|(a, b, c)| intersection(a, b, c))
            .map(|c| priority(c))
            .sum::<u32>()
    );
}
