const INPUT: &str = include_str!("real_input.txt");
const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() {
    // part 1
    {
        let mut part1 = 0;
        let pairs = INPUT.split("\n\n");
        for (index, pair) in pairs.enumerate() {
            let (left_str, right_str) = pair.split_once("\n").unwrap();
            let correct =
                Packet::verify_order(&left_str.parse().unwrap(), &right_str.parse().unwrap());
            if let PacketOrdering::Correct = correct {
                part1 += index + 1;
            }
        }
        dbg!(part1);
    }

    // part 2
    {
        let dividers: Vec<Packet> = ["[[2]]", "[[6]]"]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut messages: Vec<Packet> = INPUT
            .lines()
            .filter(|l| !l.is_empty())
            .map(|line| line.parse().unwrap())
            .collect();
        messages.append(&mut dividers.clone());
        messages.sort();

        let part2: usize = dividers
            .iter()
            .map(|d| {
                messages
                    .iter()
                    .enumerate()
                    .find(|(_, p)| *p == d)
                    .map(|(idx, _)| idx + 1)
                    .unwrap()
            })
            .product();
        dbg!(part2);
    }
}

#[derive(Debug)]
enum PacketOrdering {
    Correct,
    Incorrect,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn verify_order(left: &Packet, right: &Packet) -> PacketOrdering {
        match (left, right) {
            (Packet::Number(l), Packet::Number(r)) if l < r => PacketOrdering::Correct,
            (Packet::Number(l), Packet::Number(r)) if l > r => PacketOrdering::Incorrect,
            (Packet::Number(_), Packet::Number(_)) => PacketOrdering::Unknown,
            (Packet::Number(_), Packet::List(_)) => {
                let l_list = Packet::List(vec![left.clone()]);
                Self::verify_order(&l_list, right)
            }
            (Packet::List(_), Packet::Number(_)) => {
                let r_list = Packet::List(vec![right.clone()]);
                Self::verify_order(left, &r_list)
            }
            (Packet::List(left_elems), Packet::List(right_elems)) => {
                for i in 0..left_elems.len().min(right_elems.len()) {
                    match Packet::verify_order(&left_elems[i], &right_elems[i]) {
                        PacketOrdering::Correct => return PacketOrdering::Correct,
                        PacketOrdering::Incorrect => return PacketOrdering::Incorrect,
                        PacketOrdering::Unknown => {}
                    }
                }

                if left_elems.len() < right_elems.len() {
                    PacketOrdering::Correct
                } else if left_elems.len() > right_elems.len() {
                    PacketOrdering::Incorrect
                } else {
                    PacketOrdering::Unknown
                }
            }
        }
    }
}

impl std::str::FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        enum States {
            Packet,
            Number,
        }

        let mut state = States::Packet;
        let mut packet_stack: Vec<Vec<Packet>> = vec![vec![]];

        let mut tail = s;

        while !tail.is_empty() {
            match state {
                States::Packet if tail.starts_with("[") => {
                    state = States::Packet;
                    packet_stack.push(vec![]);
                    tail = &tail[1..];
                }
                States::Packet if tail.starts_with("]") => {
                    state = States::Packet;
                    let packet = Packet::List(packet_stack.pop().unwrap());
                    packet_stack.last_mut().unwrap().push(packet);
                    tail = &tail[1..];
                }
                States::Packet if tail.starts_with(",") => {
                    tail = &tail[1..];
                }
                States::Packet => {
                    state = States::Number;
                }
                States::Number => {
                    let mut num_str = &tail[0..0];
                    while tail[num_str.len()..].starts_with(&DIGITS) {
                        num_str = &tail[0..num_str.len() + 1];
                    }
                    packet_stack
                        .last_mut()
                        .unwrap()
                        .push(Packet::Number(num_str.parse().unwrap()));
                    tail = &tail[num_str.len()..];
                    state = States::Packet;
                }
            }
        }

        Ok(Packet::List(packet_stack.pop().unwrap()))
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match Packet::verify_order(self, other) {
            PacketOrdering::Correct => Some(std::cmp::Ordering::Less),
            PacketOrdering::Incorrect => Some(std::cmp::Ordering::Greater),
            PacketOrdering::Unknown => None,
        }
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}
