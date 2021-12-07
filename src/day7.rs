fn main() {
    let input = include_str!("day7input.txt").trim();
    let mut crabs: Vec<i32> = input.split(',').filter_map(|x| x.parse().ok()).collect();

    crabs.sort_unstable();
    let median = if crabs.len() % 2 == 0 {
        (crabs[crabs.len() / 2] + crabs[crabs.len() / 2 - 1]) / 2
    } else {
        crabs[crabs.len() / 2]
    };
    let fuel = crabs.iter().map(|x| (*x - median).abs()).sum::<i32>();
    println!("{}", fuel);

    // 2 star
    let average = (crabs.iter().sum::<i32>() as f32 / crabs.len() as f32) as i32;
    let fuel = crabs
        .iter()
        .map(|x| (*x - average).abs() * ((x - average).abs() + 1) / 2)
        .sum::<i32>();
    println!("{}", fuel);
}
