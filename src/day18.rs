#[derive(Copy, Clone)]
enum NumberKind {
    Regular(u64),
    Pair,
}

#[derive(Clone)]
struct Number(Vec<Option<NumberKind>>);

impl Number {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn set(&mut self, index: usize, kind: NumberKind) {
        if index >= self.0.len() {
            self.0.resize(index + 1, None);
        }
        self.0[index] = Some(kind);
    }

    fn add(a: &Number, b: &Number) -> Number {
        let mut result = Number::new();
        result.set(0, NumberKind::Pair);
        result.set_child(1, a, 0);
        result.set_child(2, b, 0);
        result
    }

    fn set_child(&mut self, index: usize, child: &Number, child_index: usize) {
        match &child.0.get(child_index).copied().flatten() {
            Some(NumberKind::Regular(n)) => self.set(index, NumberKind::Regular(*n)),
            Some(NumberKind::Pair) => {
                self.set(index, NumberKind::Pair);
                self.set_child(2 * index + 1, child, 2 * child_index + 1);
                self.set_child(2 * index + 2, child, 2 * child_index + 2);
            }
            None => {}
        }
    }

    fn reduce(&mut self) {
        loop {
            if !self.explode(0, 0) && !self.split(0) {
                break;
            }
        }
    }

    fn explode(&mut self, index: usize, depth: usize) -> bool {
        if self.0.get(index).copied().flatten().is_none() {
            return false;
        }
        if depth == 4 && matches!(self.0[index], Some(NumberKind::Pair)) {
            if let Some(left_index) = self.get_left_neighbour(2 * index + 1) {
                let new_val = match (self.0[left_index], self.0[2 * index + 1]) {
                    (Some(NumberKind::Regular(a)), Some(NumberKind::Regular(b))) => a + b,
                    _ => unreachable!(),
                };
                self.0[left_index] = Some(NumberKind::Regular(new_val));
                self.0[2 * index + 1] = None;
            }
            if let Some(right_index) = self.get_right_neighbour(2 * index + 2) {
                let new_val = match (self.0[right_index], self.0[2 * index + 2]) {
                    (Some(NumberKind::Regular(a)), Some(NumberKind::Regular(b))) => a + b,
                    _ => unreachable!(),
                };
                self.0[right_index] = Some(NumberKind::Regular(new_val));
                self.0[2 * index + 1] = None;
            }
            self.0[index] = Some(NumberKind::Regular(0));
            true
        } else {
            self.explode(2 * index + 1, depth + 1) || self.explode(2 * index + 2, depth + 1)
        }
    }

    fn split(&mut self, index: usize) -> bool {
        match self.0.get(index).copied().flatten() {
            None => false,
            Some(NumberKind::Regular(n)) if n > 9 => {
                self.0[index] = Some(NumberKind::Pair);
                let left = n / 2;
                self.set(2 * index + 1, NumberKind::Regular(left));
                self.set(2 * index + 2, NumberKind::Regular(n - left));
                true
            }
            _ => self.split(2 * index + 1) || self.split(2 * index + 2),
        }
    }

    fn get_left_neighbour(&self, mut index: usize) -> Option<usize> {
        loop {
            if index == 0 {
                return None;
            }
            if index % 2 == 0 {
                index -= 1;
                break;
            }
            index /= 2;
        }
        while let Some(NumberKind::Pair) = self.0[index] {
            index = 2 * index + 2;
        }
        Some(index)
    }

    fn get_right_neighbour(&self, mut index: usize) -> Option<usize> {
        loop {
            if index == 0 {
                return None;
            }
            if index % 2 != 0 {
                index += 1;
                break;
            }
            index = (index - 1) / 2;
        }
        while let Some(NumberKind::Pair) = self.0[index] {
            index = 2 * index + 1;
        }
        Some(index)
    }

    fn magnitude(&self, index: usize) -> u64 {
        match self.0.get(index).copied().flatten() {
            Some(NumberKind::Regular(n)) => n,
            Some(NumberKind::Pair) => {
                3 * self.magnitude(2 * index + 1) + 2 * self.magnitude(2 * index + 2)
            }
            _ => 0,
        }
    }
}

fn main() {
    let input = include_str!("day18input.txt");
    let numbers: Vec<Number> = input
        .lines()
        .map(|line| {
            let mut number = Number::new();
            let _ = parse(line, &mut number, 0);
            number
        })
        .collect();

    let mut result = numbers[0].clone();
    for number in numbers.iter().skip(1) {
        result = Number::add(&result, number);
        result.reduce();
    }
    println!("{}", result.magnitude(0));

    // 2 star
    let mut max = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            let a = &numbers[i];
            let b = &numbers[j];
            let mut result = Number::add(a, b);
            result.reduce();
            max = u64::max(max, result.magnitude(0));
        }
    }
    println!("{}", max);
}

fn parse<'a>(input: &'a str, number: &mut Number, index: usize) -> &'a str {
    match input.chars().next().unwrap() {
        '0'..='9' => {
            let number_end = input.find(|c: char| !c.is_numeric()).unwrap();
            let digit = &input[0..number_end].parse::<u64>().unwrap();
            number.set(index, NumberKind::Regular(*digit));
            &input[number_end..]
        }
        _ => {
            let input = &input[1..];
            let rest = parse(input, number, 2 * index + 1);
            let rest = parse(&rest[1..], number, 2 * index + 2);
            number.0[index] = Some(NumberKind::Pair);
            &rest[1..]
        }
    }
}
