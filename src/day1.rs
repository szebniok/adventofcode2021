fn main() {
    let string = include_str!("day1input.txt");
    let lines: Vec<u32> = string.lines().map(|x| x.parse().unwrap()).collect();

    println!("{}", lines.windows(2).filter(|xs| xs[0] < xs[1]).count());

    // 2 star
    println!(
        "{}",
        lines
            .windows(3)
            .map(|xs| xs.iter().sum())
            .collect::<Vec<u32>>()
            .windows(2)
            .filter(|xs| xs[0] < xs[1])
            .count()
    );
}
