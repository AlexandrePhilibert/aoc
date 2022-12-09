use std::collections::HashSet;

type Coordinate = (i32, i32);

struct Rope {
    head: Coordinate,
    tail: Coordinate,
}

impl Rope {
    fn translate(&mut self, direction: &Direction) {
        let mut dx = 0;
        let mut dy = 0;

        match direction {
            Direction::Up => dy = 1,
            Direction::Right => dx = 1,
            Direction::Down => dy = -1,
            Direction::Left => dx = -1,
        }

        let old_head = self.head;
        self.head = (self.head.0 + dx, self.head.1 + dy);

        if (self.head.0 - self.tail.0).abs() == 2 || (self.head.1 - self.tail.1).abs() == 2 {
            self.tail = old_head;
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "U" => Direction::Up,
        "R" => Direction::Right,
        "D" => Direction::Down,
        "L" => Direction::Left,
        _ => unreachable!(),
    }
}

fn main() {
    let mut positions = HashSet::new();
    let motions: Vec<(Direction, i32)> = include_str!("../input.txt")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(d, n)| (parse_direction(d), n.parse::<i32>().unwrap()))
        .collect();

    let mut rope = Rope {
        head: (0, 0),
        tail: (0, 0),
    };
    positions.insert(rope.tail);

    for motion in motions {
        for _ in 1..=motion.1 {
            rope.translate(&motion.0);
            positions.insert(rope.tail);
        }
    }

    println!("{:?}", positions.len());
}
