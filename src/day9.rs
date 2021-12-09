use std::collections::HashSet;

fn main() {
    let input = include_str!("day9input.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|x| x.to_digit(10)).collect())
        .collect();

    let mut danger = 0;
    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            let mut t = [(y - 1, x), (y, x - 1), (y + 1, x), (y, x + 1)]
                .into_iter()
                .filter(|(y, x)| {
                    (0..grid.len() as isize).contains(y) && (0..grid[0].len() as isize).contains(x)
                });
            let value = grid[y as usize][x as usize];
            if t.all(|(y, x)| value < grid[y as usize][x as usize]) {
                danger += 1 + value;
            }
        }
    }
    println!("{}", danger);

    // 2 star
    let mut basin_sizes = Vec::new();
    for y in 0..grid.len() as isize {
        for x in 0..grid[0].len() as isize {
            let mut neighbours = [(y - 1, x), (y, x - 1), (y + 1, x), (y, x + 1)]
                .into_iter()
                .filter(|(y, x)| {
                    (0..grid.len() as isize).contains(y) && (0..grid[0].len() as isize).contains(x)
                });
            let value = grid[y as usize][x as usize];
            if neighbours.all(|(y, x)| value < grid[y as usize][x as usize]) {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                basin_sizes.push(get_basin_size(x as usize, y as usize, &grid, &mut visited));
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    println!("{}", basin_sizes[0] * basin_sizes[1] * basin_sizes[2]);
}

fn get_basin_size(
    x: usize,
    y: usize,
    grid: &[Vec<u32>],
    visited: &mut HashSet<(usize, usize)>,
) -> u32 {
    visited.insert((x, y));
    let value = grid[y][x];
    if value == 9 {
        return 0;
    }
    let mut result = 1;
    if y > 0 && value < grid[y - 1][x] && !visited.contains(&(x, y - 1)) {
        result += get_basin_size(x, y - 1, grid, visited);
    }
    if y < grid.len() - 1 && value < grid[y + 1][x] && !visited.contains(&(x, y + 1)) {
        result += get_basin_size(x, y + 1, grid, visited);
    }
    if x > 0 && value < grid[y][x - 1] && !visited.contains(&(x - 1, y)) {
        result += get_basin_size(x - 1, y, grid, visited);
    }
    if x < grid[0].len() - 1 && value < grid[y][x + 1] && !visited.contains(&(x + 1, y)) {
        result += get_basin_size(x + 1, y, grid, visited);
    }
    result
}
