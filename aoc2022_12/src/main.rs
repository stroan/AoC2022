use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let grid: Grid = INPUT.parse().unwrap();

    // part 1
    {
        let part1 = grid.path_len();
        dbg!(part1);
    }

    // part 2
    {
        let mut grid = grid.clone();
        let mut part2 = std::usize::MAX;
        for y in 0..grid.height {
            for x in 0..grid.width {
                let test_start = (x, y);
                if *grid.cells.get(&test_start).unwrap() == b'a' {
                    grid.start = test_start;
                    part2 = grid.path_len().min(part2);
                }
            }
        }
        dbg!(part2);
    }
}

type Position = (i32, i32);
type Cell = u8;

#[derive(Debug, Clone)]
struct Grid {
    cells: HashMap<Position, Cell>,

    width: i32,
    height: i32,

    start: Position,
    end: Position,
}

impl Grid {
    fn path_len(&self) -> usize {
        const DIRS: [Position; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut best_link: HashMap<Position, (usize, Position)> = HashMap::default();
        let mut boundary = BinaryHeap::default();
        let mut boundary_set: HashSet<Position> = HashSet::default();

        boundary.push(Reverse((0, self.start)));
        boundary_set.insert(self.start);

        while !boundary.is_empty() {
            let (distance, position) = boundary.pop().unwrap().0;
            boundary_set.remove(&position);

            if position == self.end {
                break;
            }

            let current_height = *self.cells.get(&position).unwrap();

            for (dx, dy) in DIRS {
                let dest_distance = distance + 1;
                let dest_position = (position.0 + dx, position.1 + dy);
                let can_move_to = self
                    .cells
                    .get(&dest_position)
                    .map(|h| *h < current_height || current_height.abs_diff(*h) <= 1)
                    .unwrap_or(false);

                let is_shorter_path = best_link
                    .get(&dest_position)
                    .map(|(previous_distance, _)| dest_distance < *previous_distance)
                    .unwrap_or(true);

                if can_move_to && is_shorter_path && !boundary_set.contains(&dest_position) {
                    best_link.insert(dest_position, (dest_distance, position));
                    boundary.push(Reverse((dest_distance, dest_position)));
                }
            }
        }

        best_link
            .get(&self.end)
            .map(|(d, _)| *d)
            .unwrap_or(std::usize::MAX)
    }
}

impl std::str::FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cells: HashMap<Position, Cell> = HashMap::default();

        let mut y = 0i32;
        let mut x = 0i32;

        let mut start = Position::default();
        let mut end = Position::default();

        for line in s.lines() {
            x = 0;
            for mut c in line.chars() {
                let pos = (x, y);
                c = match c {
                    'S' => {
                        start = pos;
                        'a'
                    }
                    'E' => {
                        end = pos;
                        'z'
                    }
                    _ => c,
                };

                let cell = c as u8;
                cells.insert(pos, cell);

                x += 1;
            }

            y += 1;
        }

        Ok(Grid {
            cells,

            width: x,
            height: y,

            start,
            end,
        })
    }
}
