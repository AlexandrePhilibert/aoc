#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32)
}

fn main() {
    let mut x = 1;
    let mut cycle = 1;
    let mut signal_strength_sum = 0;

    let mut compute_cycle = |x| {
        if cycle % 40 == 20 {
            signal_strength_sum += cycle * x;
        }
        cycle += 1;
    };

    include_str!("../input.txt")
                        .lines()
                        .map(|line| line.split_whitespace())
                        .map(|mut w| {
                            let instruction_name = w.next().expect("Expected instruction name");
                            let value = w.next();

                            match instruction_name {
                                "noop" => Instruction::Noop,
                                "addx" => Instruction::AddX(value.unwrap().parse().unwrap()),
                                _ => panic!("Unknown instruction"),
                            }
                        }).for_each(|instruction| {
                            match instruction {
                                Instruction::Noop => {
                                    compute_cycle(x);
                                },
                                Instruction::AddX(v) => {
                                    compute_cycle(x);
                                    compute_cycle(x);
                                    x += v;
                                }
                            }
                        });

    println!("{:?}", signal_strength_sum);
}
