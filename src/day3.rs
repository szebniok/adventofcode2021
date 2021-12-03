fn main() {
    let string = include_str!("day3input.txt");
    let lines: Vec<&str> = string.lines().collect();

    let bit_len = lines[0].len();
    let mut occurences = vec![[0u32, 0u32]; lines[0].len()];

    for line in lines.iter() {
        for (i, c) in line.chars().enumerate() {
            let val = if c == '0' { 0 } else { 1 };
            occurences[i][val] += 1;
        }
    }

    let gamma = occurences
        .iter()
        .rev()
        .enumerate()
        .map(|(i, [zeros, ones])| if ones > zeros { 2u32.pow(i as u32) } else { 0 })
        .sum::<u32>();
    let epsilon = (!gamma << (32 - bit_len)) >> (32 - bit_len);
    println!("{}", gamma * epsilon);

    // 2 star
    let f = |one_predicate: &dyn Fn(usize, usize) -> bool| {
        let mut n = 0;
        let mut lines = (&lines).clone();
        while lines.len() > 1 {
            let ones_count = lines
                .iter()
                .map(|x| x.chars().nth(n).unwrap())
                .filter(|x| *x == '1')
                .count();
            let zeros_count = lines.len() - ones_count;
            let criteria = if one_predicate(zeros_count, ones_count) {
                '1'
            } else {
                '0'
            };
            lines.retain(|x| x.chars().nth(n).unwrap() == criteria);
            n += 1;
        }
        u32::from_str_radix(lines[0], 2).unwrap()
    };

    let oxygen = f(&|zeroes, ones| ones >= zeroes);
    let co2 = f(&|zeroes, ones| ones < zeroes);

    println!("{}", oxygen * co2);
}
