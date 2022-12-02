const INPUT: &str = include_str!("real_input.txt");

#[derive(Copy, Clone, Debug)]
struct Move(u32);

impl Move {
    fn from_abc(letter: &str) -> Self {
        Self(letter.chars().next().unwrap() as u32 - 'A' as u32)
    }

    fn from_xyz(letter: &str) -> Self {
        Self(letter.chars().next().unwrap() as u32 - 'X' as u32)
    }

    fn relative_xyz(&self, letter: &str) -> Self {
        let rel = letter.chars().next().unwrap() as u32 - 'X' as u32;
        Self((self.0 + 2 + rel) % 3)
    }

    fn score(&self) -> u32 {
        self.0 + 1
    }

    fn fight(&self, other: &Self) -> u32 {
        (self.0 + 4 - other.0) % 3
    }
}

fn main() {
    let lines: Vec<&str> = INPUT.lines().collect();

    // part 1
    {
        let moves: Vec<(Move, Move)> = lines
            .iter()
            .map(|line| {
                let (left, right) = line.split_once(' ').unwrap();
                (Move::from_abc(left), Move::from_xyz(right))
            })
            .collect();

        let part1_score = score(&moves);
        dbg!(part1_score);
    }

    // part 2
    {
        let moves: Vec<(Move, Move)> = lines
            .iter()
            .map(|line| {
                let (left, right) = line.split_once(' ').unwrap();
                let their_move = Move::from_abc(left);
                (their_move, their_move.relative_xyz(right))
            })
            .collect();

        let part2_score = score(&moves);
        dbg!(part2_score);
    }
}

fn score(moves: &[(Move, Move)]) -> u32 {
    moves
        .iter()
        .map(|(move_a, move_x)| move_x.score() + (move_x.fight(move_a) * 3))
        .sum()
}
