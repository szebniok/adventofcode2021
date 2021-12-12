use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("day12input.txt");
    let edges = input.lines().map(|line| {
        let mut s = line.split('-');
        (s.next().unwrap(), s.next().unwrap())
    });
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (from, to) in edges {
        map.entry(from).or_insert_with(Vec::new).push(to);
        map.entry(to).or_insert_with(Vec::new).push(from);
    }

    let result = get_paths_num(&map, "start", HashSet::new(), false, false);
    println!("{}", result);

    // 2 star
    let result = get_paths_num(&map, "start", HashSet::new(), true, false);
    println!("{}", result);
}

fn get_paths_num(
    map: &HashMap<&str, Vec<&str>>,
    from: &str,
    mut visited: HashSet<String>,
    part2: bool,
    twice: bool,
) -> u32 {
    if from == "end" {
        return 1;
    }
    if from.chars().next().unwrap().is_lowercase() {
        visited.insert(from.to_owned());
    }
    let mut result = 0;
    for &dest in map.get(from).unwrap() {
        if !visited.contains(dest) {
            result += get_paths_num(map, dest, visited.clone(), part2, twice);
        } else if part2 && dest != "start" && !twice {
            result += get_paths_num(map, dest, visited.clone(), part2, true);
        }
    }
    result
}
