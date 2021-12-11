use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("day11input.txt");
    let mut grid: [[u32; 10]; 10] = input
        .lines()
        .filter_map(|line| {
            line.chars()
                .filter_map(|x| x.to_digit(10))
                .collect::<Vec<_>>()
                .try_into()
                .ok()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let mut flashed: HashSet<(usize, usize)> = HashSet::new();
    let mut incremented: VecDeque<(usize, usize)> = VecDeque::new();
    let mut count = 0;
    let mut step = 0;
    loop {
        incremented.extend((0..10).flat_map(|y| (0..10).map(move |x| (x, y))));
        while let Some((x, y)) = incremented.pop_front() {
            if flashed.contains(&(x, y)) {
                continue;
            };
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                count += 1;
                grid[y][x] = 0;
                flashed.insert((x, y));
                let neighbours = {
                    let (x, y) = (x as isize, y as isize);
                    [
                        (x - 1, y - 1),
                        (x, y - 1),
                        (x + 1, y - 1),
                        (x - 1, y),
                        (x + 1, y),
                        (x - 1, y + 1),
                        (x, y + 1),
                        (x + 1, y + 1),
                    ]
                    .into_iter()
                    .filter(|(x, y)| {
                        (0..grid.len() as isize).contains(y)
                            && (0..grid[0].len() as isize).contains(x)
                    })
                    .filter_map(|(x, y)| {
                        (!flashed.contains(&(x as usize, y as usize)))
                            .then(|| (x as usize, y as usize))
                    })
                };
                incremented.extend(neighbours);
            }
        }
        step += 1;
        if step == 100 {
            println!("{}", count);
        }

        // 2 star
        if flashed.len() == 100 {
            println!("{}", step);
            break;
        }
        flashed.clear();
    }
}
