fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 26 + 1,
        _ => unreachable!("How ?"),
    }
}

fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
            .lines()
            .map(|l| l.split_at(l.len() / 2))
            .map(|(first, second)| first
                .chars()
                .into_iter()
                .find(|c| second.contains(*c))
                .unwrap())
            .map(|c| priority(c))
            .sum::<u32>()
    );
}
