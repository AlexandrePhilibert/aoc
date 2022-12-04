use itertools::Itertools;

fn main() {
    let lines = include_str!("../input.txt")
        .lines()
        .map(|l| l.split(",").collect::<Vec<&str>>())
        .map(|l| {
            l.iter()
                .map(|p| {
                    let (start, end) = p
                        .split("-")
                        .map(|v| v.parse::<u32>().unwrap())
                        .collect_tuple()
                        .unwrap();

                    start..=end
                })
                .collect::<Vec<_>>()
        })
        .map(|pair| {
            (pair[0].contains(&pair[1].start()) || pair[0].contains(&pair[1].end()))
                || (pair[1].contains(&pair[0].start()) || pair[1].contains(&pair[0].end()))
        })
        .fold(0, |acc, b| acc + b as u32);

    println!("{:#?}", lines);
}
