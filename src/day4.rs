struct Board {
    numbers: [u8; 5 * 5],
    marks: [bool; 5 * 5],
    won: bool,
}

impl Board {
    const COLUMNS_START: [usize; 5] = [0, 1, 2, 3, 4];
    const ROWS_START: [usize; 5] = [0, 5, 10, 15, 20];

    fn parse(input: &[&str]) -> Self {
        let numbers: [u8; 5 * 5] = input
            .iter()
            .flat_map(|x| x.split(' '))
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            numbers,
            marks: [false; 5 * 5],
            won: false,
        }
    }

    fn update(&mut self, number: u8) -> bool {
        if self.won {
            return false;
        }
        if let Some(index) = self.numbers.iter().position(|x| *x == number) {
            self.marks[index] = true;
            let won = self.check_win_condition();
            self.won = won;
            won
        } else {
            false
        }
    }

    fn check_win_condition(&self) -> bool {
        let columns = Self::COLUMNS_START
            .into_iter()
            .any(|s| (s..25).step_by(5).all(|i| self.marks[i]));
        let rows = Self::ROWS_START
            .into_iter()
            .any(|s| (s..s + 5).all(|i| self.marks[i]));
        columns || rows
    }

    fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .zip(self.marks.iter())
            .filter_map(|(num, mark)| (!*mark).then(|| *num as u32))
            .sum()
    }
}

fn main() {
    let string = include_str!("day4input.txt");
    let lines: Vec<&str> = string.lines().collect();

    let draws: Vec<u8> = lines[0].split(',').filter_map(|x| x.parse().ok()).collect();
    let encoded_boards = lines[2..].chunks_exact(6);
    let mut boards: Vec<Board> = encoded_boards.map(|x| Board::parse(x)).collect();
    let mut result = 0;

    'draws: for draw in draws.iter() {
        for board in boards.iter_mut() {
            let won = board.update(*draw);
            if won {
                result = board.unmarked_sum() * *draw as u32;
                break 'draws;
            }
        }
    }
    println!("{}", result);

    // 2 star
    for draw in draws {
        for board in boards.iter_mut() {
            let won = board.update(draw);
            if won {
                result = board.unmarked_sum() * draw as u32;
            }
        }
    }
    println!("{}", result);
}
