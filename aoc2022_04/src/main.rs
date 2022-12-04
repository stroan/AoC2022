use std::str::FromStr;

const INPUT: &str = include_str!("real_input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Assignment {
    min: u32,
    max: u32,
}

impl Assignment {
    fn union(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    fn size(&self) -> u32 {
        self.max - self.min
    }
}

impl FromStr for Assignment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min_str, max_str) = s.split_once("-").ok_or(())?;
        Ok(Assignment {
            min: min_str.parse::<u32>().map_err(|_| ())?,
            max: max_str.parse::<u32>().map_err(|_| ())? + 1,
        })
    }
}

fn count_with_condition<F: Fn(Assignment, Assignment, Assignment) -> bool>(f: F) -> u32 {
    INPUT
        .lines()
        .filter(|line| {
            let (l_str, r_str) = line.split_once(",").unwrap();
            let l: Assignment = l_str.parse().unwrap();
            let r: Assignment = r_str.parse().unwrap();
            let union = l.union(r);
            f(l, r, union)
        })
        .count() as u32
}

fn main() {
    // Part 1
    {
        let part1 = count_with_condition(|left, right, union| union == left || union == right);
        dbg!(part1);
    }

    // Part 2
    {
        let part2 =
            count_with_condition(|left, right, union| left.size() + right.size() > union.size());
        dbg!(part2);
    }
}
