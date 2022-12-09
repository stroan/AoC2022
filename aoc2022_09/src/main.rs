use std::collections::HashSet;

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    // part 1
    {
        let part1 = rope_sim(2);
        dbg!(part1);
    }

    // part 2
    {
        let part2 = rope_sim(10);
        dbg!(part2);
    }
}

fn rope_sim(len: usize) -> usize {
    let mut knots = vec![Vec2 { x: 0, y: 0 }; len];

    let mut visited: HashSet<Vec2> = HashSet::new();
    visited.insert(Vec2 { x: 0, y: 0 });

    for line in INPUT.lines() {
        let (direction, count_str) = line.split_once(" ").unwrap();
        let count: usize = count_str.parse().unwrap();

        let dir_vec = match direction {
            "U" => Vec2 { x: 0, y: 1 },
            "D" => Vec2 { x: 0, y: -1 },
            "L" => Vec2 { x: -1, y: 0 },
            "R" => Vec2 { x: 1, y: 0 },
            _ => panic!("Unknown dir"),
        };

        for _ in 0..count {
            knots[0] = knots[0] + dir_vec;
            for i in 1..len {
                knots[i] = drag_tail(knots[i - 1], knots[i]);
            }
            visited.insert(knots[len - 1]);
        }
    }

    visited.len()
}

fn drag_tail(head: Vec2, tail: Vec2) -> Vec2 {
    let delta = head - tail;

    if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
        tail
    } else {
        let step = Vec2 {
            x: delta.x.min(1).max(-1),
            y: delta.y.min(1).max(-1),
        };
        tail + step
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
