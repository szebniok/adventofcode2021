fn main() {
    let input = include_str!("day17input.txt").trim();
    let (x1, x2, y1, y2): (i32, i32, i32, i32) = {
        let (_, coords) = input.split_once(": ").unwrap();
        let (xs, ys) = coords.split_once(", ").unwrap();
        let (x1, x2) = &xs[2..].split_once("..").unwrap();
        let (y1, y2) = &ys[2..].split_once("..").unwrap();
        (
            x1.parse().unwrap(),
            x2.parse().unwrap(),
            y1.parse().unwrap(),
            y2.parse().unwrap(),
        )
    };

    let max_v = y1.abs() - 1;
    let max_y = (max_v * (max_v + 1)) / 2;
    println!("{}", max_y);

    // 2 star
    let x_vel_min = (-1 + ((1 + 8 + x1) as f32).sqrt() as i32) / 2;
    let mut result = 0;
    for x_vel_start in x_vel_min..=x2 {
        'y: for mut y_vel in y1..max_y {
            let mut x_vel = x_vel_start;
            let (mut x, mut y) = (0, 0);
            while x <= x2 {
                if x1 <= x && y1 <= y && y <= y2 {
                    result += 1;
                    continue 'y;
                }
                if y < y1 {
                    continue 'y;
                }
                x += x_vel;
                y += y_vel;
                x_vel = i32::max(x_vel - 1, 0);
                y_vel -= 1;
            }
        }
    }
    println!("{}", result);
}
