use std::collections::HashMap;

fn main() {
    let input = include_str!("day5input.txt");
    let lines: Vec<((i32, i32), (i32, i32))> = input
        .lines()
        .map(|line| {
            let s = line
                .split(|x: char| !x.is_numeric())
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<_>>();
            ((s[0], s[1]), (s[2], s[3]))
        })
        .collect();

    let mut occurences: HashMap<(i32, i32), u32> = HashMap::new();
    let horizontal_or_vertical = lines
        .iter()
        .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
        .copied()
        .collect::<Vec<_>>();

    let dir = |s: i32, d: i32| if d == s { 0 } else { (d - s) / (d - s).abs() };
    for &((x1, y1), (x2, y2)) in horizontal_or_vertical.iter() {
        let x_dir = dir(x1, x2);
        let y_dir = dir(y1, y2);
        let (mut x, mut y) = (x1, y1);
        let steps_num = i32::max((x1 - x2).abs(), (y1 - y2).abs());
        for _ in 0..=steps_num {
            *occurences.entry((x, y)).or_insert(0) += 1;
            x += x_dir;
            y += y_dir;
        }
    }
    println!("{}", occurences.values().filter(|x| **x >= 2).count());

    // 2 star
    occurences.clear();
    for &((x1, y1), (x2, y2)) in lines.iter() {
        let x_dir = dir(x1, x2);
        let y_dir = dir(y1, y2);
        let (mut x, mut y) = (x1, y1);
        let steps_num = i32::max((x1 - x2).abs(), (y1 - y2).abs());
        for _ in 0..=steps_num {
            *occurences.entry((x, y)).or_insert(0) += 1;
            x += x_dir;
            y += y_dir;
        }
    }
    println!("{}", occurences.values().filter(|x| **x >= 2).count());
}
