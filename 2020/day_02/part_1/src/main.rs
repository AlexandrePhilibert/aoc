use regex::Regex;

fn main() {
    let regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| regex.captures(l).unwrap())
            .filter(|list| {
                let min = list[1].parse().unwrap();
                let max = list[2].parse().unwrap();
                let character = &list[3];
                let password = &list[4];
                let count = password.matches(&character).count();

                count >= min && count <= max
            })
            .count()
    )
}
