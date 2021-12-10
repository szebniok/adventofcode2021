use std::collections::VecDeque;

fn main() {
    let input = include_str!("day10input.txt");
    let lines: Vec<&str> = input.lines().collect();

    let opening_brace = |c: char| match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => '<',
    };
    let mut score = 0;
    for line in lines.iter() {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push_back(c);
            } else if stack.pop_back() != Some(opening_brace(c)) {
                score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    _ => 25137,
                };
                break;
            }
        }
    }
    println!("{}", score);

    // 2 star
    let mut scores: Vec<u64> = Vec::new();
    'line: for line in lines.iter() {
        let mut stack: VecDeque<char> = VecDeque::new();
        for c in line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                stack.push_back(c);
            } else if stack.pop_back() != Some(opening_brace(c)) {
                continue 'line;
            }
        }
        let mut line_score = 0;
        while let Some(opening) = stack.pop_back() {
            line_score *= 5;
            line_score += match opening {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                _ => 4,
            };
        }
        scores.push(line_score);
    }
    let middle_index = scores.len() / 2;
    let (_, middle, _) = scores.select_nth_unstable(middle_index);
    println!("{}", middle);
}
