//fn visible_on_line<'a, I, O>(line: I, visible_line: &mut O)
//where
//    I: Iterator<Item = &'a i32>,
//    O: Iterator<Item = &'a mut bool>,
//{
//    let mut max = -1;
//    line.for_each(|item| {
//        let visible_iter = visible_line.next().unwrap();
//
//        if item > &max {
//            max = *item;
//            *visible_iter = true;
//        }
//    })
//}

fn visible_on_line<'a, I>(line: I, visible_line: &mut Vec<bool>)
where
    I: Iterator<Item = (usize, &'a i32)>,
{
    let mut max = -1;
    line.for_each(|(index, item)| {
        if item > &max {
            max = *item;
            visible_line[index] = true;
        }
    })
}

fn main() {
    let tree_map: Vec<Vec<i32>> = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let row_count = tree_map.len();
    let column_count = tree_map[0].len();
    let mut visible_map = vec![vec![false; column_count]; row_count];

    for y in 0..row_count {
        // left to right
        visible_on_line(tree_map[y].iter().enumerate(), &mut visible_map[y]);
        // right to left
        visible_on_line(tree_map[y].iter().enumerate().rev(), &mut visible_map[y]);
    }

    // top to bottom
    for x in 0..column_count {
        let mut max = -1;

        for y in 0..row_count {
            if tree_map[y][x] > max {
                max = tree_map[y][x];
                visible_map[y][x] = true;
            }
        }
    }

    // bottom to top
    for x in 0..column_count {
        let mut max = -1;

        for y in (0..row_count).rev() {
            if tree_map[y][x] > max {
                max = tree_map[y][x];
                visible_map[y][x] = true;
            }
        }
    }

    println!(
        "{:?}",
        visible_map
            .iter()
            .flatten()
            .filter(|visible| **visible)
            .count()
    );
}
