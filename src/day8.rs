use std::collections::HashSet;

fn main() {
    let input = include_str!("day8input.txt").trim();
    type Digit = HashSet<char>;
    let lines: Vec<(Vec<Digit>, Vec<Digit>)> = input
        .lines()
        .map(|line| {
            let (signals, digits) = line.split_once(" | ").unwrap();
            (
                signals.split(' ').map(|x| x.chars().collect()).collect(),
                digits.split(' ').map(|x| x.chars().collect()).collect(),
            )
        })
        .collect();

    let count_1478 = lines
        .iter()
        .map(|x| {
            x.1.iter()
                .filter(|x| [2, 3, 4, 7].contains(&x.len()))
                .count()
        })
        .sum::<usize>();
    println!("{}", count_1478);

    // 2 star
    let sum = lines
        .iter()
        .map(|(signals, digits)| {
            let d1 = signals.iter().find(|x| x.len() == 2).unwrap();
            let d4 = signals.iter().find(|x| x.len() == 4).unwrap();
            let d7 = signals.iter().find(|x| x.len() == 3).unwrap();
            let d8 = signals.iter().find(|x| x.len() == 7).unwrap();

            let mut p0_6_9 = signals.iter().filter(|x| x.len() == 6);
            let d9 = p0_6_9
                .find(|x| !d4.contains(d8.difference(x).next().unwrap()))
                .unwrap();
            let mut p0_6 = signals.iter().filter(|x| x.len() == 6 && *x != d9);
            let d0 = p0_6.find(|x| d1.is_subset(x)).unwrap();
            let mut p0_6 = signals.iter().filter(|x| x.len() == 6 && *x != d9);
            let d6 = p0_6.find(|x| *x != d0).unwrap();

            let mut p_2_3_5 = signals.iter().filter(|x| x.len() == 5);
            let d3 = p_2_3_5.find(|x| d1.is_subset(x)).unwrap();

            let mut p_2_5 = signals.iter().filter(|x| x.len() == 5 && *x != d3);
            let d5 = p_2_5.find(|x| d6.is_superset(x)).unwrap();
            let mut p_2_5 = signals.iter().filter(|x| x.len() == 5 && *x != d3);
            let d2 = p_2_5.find(|x| *x != d5).unwrap();

            let map = [d0, d1, d2, d3, d4, d5, d6, d7, d8, d9];

            let mut result = 0;
            for digit in digits {
                result *= 10;
                result += map.iter().position(|x| *x == digit).unwrap() as u32;
            }
            result
        })
        .sum::<u32>();

    println!("{}", sum);
}
