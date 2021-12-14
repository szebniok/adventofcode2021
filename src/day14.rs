use std::collections::HashMap;

fn main() {
    let input = include_str!("day14input.txt");
    let mut lines = input.lines();
    let chain = lines.next().unwrap();
    let rules: HashMap<(char, char), char> = lines
        .skip(1)
        .filter_map(|line| {
            let (pair, element) = line.split_once(" -> ")?;
            let mut pair_chars = pair.chars();
            Some((
                (pair_chars.next()?, pair_chars.next()?),
                element.chars().next()?,
            ))
        })
        .collect();

    let mut result: HashMap<(char, char), u64> = HashMap::new();
    for pair in chain.as_bytes().windows(2) {
        *result
            .entry((pair[0] as char, pair[1] as char))
            .or_insert(0) += 1;
    }

    let print = |result: &HashMap<(char, char), u64>| {
        let mut occ: HashMap<char, u64> = HashMap::new();
        for (&(left, _), count) in result {
            *occ.entry(left).or_insert(0) += count;
        }
        *occ.entry(chain.chars().rev().next().unwrap()).or_insert(0) += 1;
        let min = occ.values().min().unwrap();
        let max = occ.values().max().unwrap();
        println!("{}", max - min);
    };

    for i in 0..40 {
        let mut new_result: HashMap<(char, char), u64> = HashMap::new();
        for (&(left, right), count) in result.iter() {
            let middle = *rules.get(&(left, right)).unwrap();
            *new_result.entry((left, middle)).or_insert(0) += count;
            *new_result.entry((middle, right)).or_insert(0) += count;
        }
        result = new_result;
        if i == 9 {
            print(&result);
        }
    }

    // 2 star
    print(&result);
}
