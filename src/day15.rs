use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

fn main() {
    let input = include_str!("day15input.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect())
        .collect();

    println!("{}", get_risk(&grid));

    // 2 star
    let (width, height) = (grid[0].len(), grid.len());
    let mut big_grid = vec![vec![0u32; width * 5]; height * 5];
    for (y, row) in grid.into_iter().enumerate() {
        for (x, col) in row.into_iter().enumerate() {
            for offset_y in 0..5 {
                for offset_x in 0..5 {
                    big_grid[y + offset_y * height][x + offset_x * width] =
                        (col + offset_y as u32 + offset_x as u32 - 1) % 9 + 1;
                }
            }
        }
    }
    println!("{}", get_risk(&big_grid));
}

fn get_risk(grid: &[Vec<u32>]) -> u32 {
    let (width, height) = (grid[0].len(), grid.len());
    let mut risk = vec![vec![u32::MAX; width]; height];
    let mut heap: BinaryHeap<Reverse<(u32, (usize, usize))>> = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    risk[0][0] = 0;
    heap.push(Reverse((0, (0, 0))));

    while let Some(Reverse((cost, (x, y)))) = heap.pop() {
        if (x, y) == (width - 1, height - 1) {
            return cost;
        }
        if cost > risk[y][x] || visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));

        let adjacent = {
            let (x, y) = (x as isize, y as isize);
            [(x, y + 1), (x + 1, y), (x - 1, y), (x, y - 1)]
                .into_iter()
                .filter(|&(x, y)| 0 <= x && x < width as isize && 0 <= y && y < height as isize)
                .map(|(x, y)| (x as usize, y as usize))
        };
        for (nx, ny) in adjacent {
            let new_cost = cost + grid[ny][nx];
            if new_cost < risk[ny][nx] {
                heap.push(Reverse((new_cost, (nx, ny))))
            }
        }
    }
    0
}
