#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32)
}

fn main() {
    let mut x = 1;
    let mut cycle = 1;
    let mut current_pixel_index = 0;

    let mut compute_cycle = |x| {
        if current_pixel_index >= x - 1 && current_pixel_index <= x + 1 {
            print!("#");
        } else {
            print!(".");
        }

        current_pixel_index = (current_pixel_index + 1) % 40;
        if current_pixel_index == 0 {
            print!("\n");
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
}
