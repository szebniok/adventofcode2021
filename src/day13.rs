fn main() {
    let input = include_str!("day13input.txt");
    let (dots_input, folds_input) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots_input
        .lines()
        .filter_map(|line| {
            let (x, y) = line.split_once(",")?;
            let (x, y) = (x.parse().ok()?, y.parse().ok()?);
            Some((x, y))
        })
        .collect();
    let max_x = *dots.iter().map(|(x, _)| x).max().unwrap();
    let max_y = *dots.iter().map(|(_, y)| y).max().unwrap();
    let mut sheet = vec![vec![0u8; max_x + 1]; max_y + 1];
    for (x, y) in dots {
        sheet[y][x] = 1;
    }
    let folds: Vec<(&str, usize)> = folds_input
        .lines()
        .filter_map(|line| {
            let position = line.split(' ').nth(2)?;
            let (direction, index) = position.split_once("=")?;
            Some((direction, index.parse().ok()?))
        })
        .collect();

    let (direction, position) = folds[0];
    let result = fold(&sheet, direction, position);
    let a: usize = result
        .iter()
        .map(|row| row.iter().filter(|&&x| x != 0).count())
        .sum();
    println!("{}", a);

    // 2 star
    let mut result = sheet;
    for (direction, position) in folds {
        result = fold(&result, direction, position);
    }
    for row in result {
        for col in row {
            print!("{}", if col != 0 { "#" } else { "." });
        }
        println!();
    }
}

fn fold(sheet: &[Vec<u8>], direction: &str, position: usize) -> Vec<Vec<u8>> {
    let (new_width, new_height) = if direction == "x" {
        (
            usize::max(position, sheet[0].len() - position - 1),
            sheet.len(),
        )
    } else {
        (
            sheet[0].len(),
            usize::max(position, sheet.len() - position - 1),
        )
    };
    let mut result = vec![vec![0u8; new_width]; new_height];
    if direction == "y" {
        for y in (new_height - position)..new_height {
            for x in 0..new_width {
                result[y][x] += sheet[y - (new_height - position)][x];
            }
        }
        for y_offset in 0..(sheet.len() - new_height - 1) {
            for x in 0..new_width {
                result[position - 1 - y_offset][x] += sheet[position + y_offset + 1][x];
            }
        }
    }
    if direction == "x" {
        for x in (new_width - position)..new_width {
            for y in 0..new_height {
                result[y][x] += sheet[y][x - (new_width - position)];
            }
        }
        for x_offset in 0..(sheet[0].len() - new_width - 1) {
            for y in 0..new_height {
                result[y][position - 1 - x_offset] += sheet[y][position + 1 + x_offset];
            }
        }
    }
    result
}
