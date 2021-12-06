fn main() {
    let input = include_str!("day6input.txt").trim();
    let input: Vec<u8> = input.split(',').filter_map(|x| x.parse().ok()).collect();

    let mut lanternfishes = [0u64; 9];
    for i in input.iter() {
        lanternfishes[*i as usize] += 1;
    }

    for _ in 0..80 {
        lanternfishes.rotate_left(1);
        lanternfishes[6] += lanternfishes[8];
    }
    println!("{}", lanternfishes.iter().sum::<u64>());

    // 2 star
    for _ in 80..256 {
        lanternfishes.rotate_left(1);
        lanternfishes[6] += lanternfishes[8];
    }
    println!("{}", lanternfishes.iter().sum::<u64>());
}
