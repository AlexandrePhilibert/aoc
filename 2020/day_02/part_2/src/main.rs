use regex::Regex;

fn main() {
    let regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|l| regex.captures(l).unwrap())
            .filter(|list| {
                let first = list[1].parse::<usize>().unwrap() - 1;
                let second = list[2].parse::<usize>().unwrap() - 1;
                let character = list[3].chars().next().unwrap();
                let password = list[4].chars().collect::<Vec<char>>();

                (password[first] != character || password[second] != character)
                    && (password[first] == character || password[second] == character)
            })
            .count()
    )
}
