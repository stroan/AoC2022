const INPUT: &str = include_str!("real_input.txt");

type ShipStacks = Vec<CargoStack>;
type CargoStack = std::collections::VecDeque<char>;

fn main() {
    // part 1
    {
        let part1 = operate_crane(|stacks, instruction| {
            for _ in 0..instruction.count {
                let cr8 = stacks[instruction.src - 1].pop_back().unwrap();
                stacks[instruction.dst - 1].push_back(cr8);
            }
        });
        dbg!(part1);
    }

    // part 2
    {
        let part2 = operate_crane(|stacks, instruction| {
            let mut tmp: CargoStack = CargoStack::default();
            for _ in 0..instruction.count {
                tmp.push_front(stacks[instruction.src - 1].pop_back().unwrap());
            }
            for _ in 0..instruction.count {
                stacks[instruction.dst - 1].push_back(tmp.pop_front().unwrap());
            }
        });
        dbg!(part2);
    }
}

fn operate_crane<F: Fn(&mut ShipStacks, &Instruction)>(f: F) -> String {
    let mut lines = INPUT.lines();

    let mut line = lines.next().unwrap();
    let mut stacks: ShipStacks = vec![CargoStack::default(); ((line.len() - 3) / 4) + 1];
    loop {
        if !line.contains("[") {
            break;
        }

        for i in 0..stacks.len() {
            let cr8 = line.as_bytes()[1 + (4 * i)] as char;
            if cr8 != ' ' {
                stacks[i].push_front(cr8);
            }
        }

        line = lines.next().unwrap();
    }
    lines.next().unwrap();

    for line in lines {
        f(&mut stacks, &line.parse().unwrap());
    }

    stacks.iter().map(|stack| stack.back().unwrap()).collect()
}

#[derive(Debug)]
struct Instruction {
    count: usize,
    src: usize,
    dst: usize,
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count_str, rest) = s[5..].split_once(" from ").ok_or(())?;
        let (src_str, dst_str) = rest.split_once(" to ").ok_or(())?;
        Ok(Self {
            count: count_str.parse().map_err(|_| ())?,
            src: src_str.parse().map_err(|_| ())?,
            dst: dst_str.parse().map_err(|_| ())?,
        })
    }
}
