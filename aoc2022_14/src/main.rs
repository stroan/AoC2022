use std::collections::HashSet;

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    // part 1
    {
        let mut part1 = 0;
        let mut cave: Cave = INPUT.parse().unwrap();
        while cave.drop_sand(false).is_some() {
            part1 += 1;
        }
        dbg!(part1);
    }

    // part 2
    {
        let mut part2 = 0;
        let mut cave: Cave = INPUT.parse().unwrap();
        while cave.drop_sand(true).is_some() {
            part2 += 1;
        }
        dbg!(part2);
    }
}

#[derive(Debug)]
struct Cave {
    rocks: HashSet<Vec2>,
    height: i32,
}

impl Cave {
    fn drop_sand(&mut self, use_floor: bool) -> Option<Vec2> {
        let mut pos = Vec2 { x: 500, y: 0 };
        const TEST_DIRS: [Vec2; 3] = [
            Vec2 { x: 0, y: 1 },
            Vec2 { x: -1, y: 1 },
            Vec2 { x: 1, y: 1 },
        ];

        loop {
            if self.rocks.contains(&pos) {
                return None;
            } else if pos.y >= self.height + 3 {
                return None;
            } else if let Some(new_pos) =
                TEST_DIRS
                    .iter()
                    .map(|&dir| dir + pos)
                    .find(|&potential_pos| {
                        !self.rocks.contains(&potential_pos)
                            && !(use_floor && potential_pos.y == (self.height + 2))
                    })
            {
                pos = new_pos;
            } else {
                self.rocks.insert(pos);
                return Some(pos);
            }
        }
    }
}

impl std::str::FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rocks = HashSet::default();
        let mut height: i32 = 0;

        for line in s.lines() {
            let mut prev = None;
            for point_str in line.split(" -> ") {
                let dest_point: Vec2 = point_str.parse().unwrap();
                if let Some(prev) = prev {
                    let delta = (dest_point - prev).clamp();
                    let mut current = prev;
                    loop {
                        let done = current == dest_point;
                        rocks.insert(current);
                        height = height.max(current.y);
                        current = current + delta;
                        if done {
                            break;
                        }
                    }
                }
                prev = Some(dest_point);
            }
        }

        Ok(Cave { rocks, height })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn clamp(self) -> Self {
        Self {
            x: self.x.clamp(-1, 1),
            y: self.y.clamp(-1, 1),
        }
    }
}

impl std::str::FromStr for Vec2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(",").ok_or(())?;
        Ok(Vec2 {
            x: x_str.parse().map_err(|_| ())?,
            y: y_str.parse().map_err(|_| ())?,
        })
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
