fn main() {
    let string = include_str!("day2input.txt");
    let lines: Vec<(&str, i32)> = string
        .lines()
        .map(|x| {
            let mut split = x.split(' ');
            (
                split.next().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let depth: i32 = lines
        .iter()
        .filter_map(|(cmd, unit)| {
            if *cmd != "forward" {
                let sign = if *cmd == "down" { 1 } else { -1 };
                Some(sign * unit)
            } else {
                None
            }
        })
        .sum();

    let position: i32 = lines
        .iter()
        .filter_map(|(cmd, unit)| (*cmd == "forward").then(|| unit))
        .sum();

    println!("{}", depth * position);

    // 2 star
    let (mut depth, mut aim) = (0, 0);

    for (cmd, unit) in lines {
        if cmd == "forward" {
            depth += unit * aim;
        } else {
            let sign = if cmd == "down" { 1 } else { -1 };
            aim += sign * unit;
        }
    }

    println!("{}", depth * position);
}
