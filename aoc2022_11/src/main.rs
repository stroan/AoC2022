use std::collections::VecDeque;

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    // part 1
    {
        let part1 = score_rounds(20, Some(3), false);
        dbg!(part1);
    }

    // part 2
    {
        let part1 = score_rounds(10000, None, true);
        dbg!(part1);
    }
}

fn score_rounds(count: u32, divisor: Option<u32>, use_multimodulus: bool) -> u64 {
    let mut monkeys = parse_monkeys();

    if use_multimodulus {
        let tests: Vec<u32> = monkeys.iter().map(|m| m.test).collect();
        for m in monkeys.iter_mut() {
            for i in m.items.iter_mut() {
                *i = i.as_multimodulus(&tests[..]);
            }
        }
    }

    for _ in 0..count {
        do_round(&mut monkeys, divisor);
    }

    let mut scores: Vec<u64> = monkeys.iter().map(|m| m.inspection_count as u64).collect();
    scores.sort();
    scores.reverse();

    scores[0..2].iter().product()
}

fn do_round(monkeys: &mut Vec<Monkey>, divisor: Option<u32>) {
    for i in 0..monkeys.len() {
        while !monkeys[i].items.is_empty() {
            monkeys[i].inspection_count += 1;

            let mut item = monkeys[i].items.pop_front().unwrap();
            (monkeys[i].operation)(&mut item, &monkeys[i].operand);
            item.div_assign(&divisor);

            let dest = if item.modulo(monkeys[i].test) == 0 {
                monkeys[i].test_true
            } else {
                monkeys[i].test_false
            } as usize;
            monkeys[dest].items.push_back(item);
        }
    }
}

fn parse_monkeys() -> Vec<Monkey> {
    let mut lines = INPUT.lines();
    let mut line = lines.next();

    let mut result = vec![];

    while line.is_some() {
        // skip id line
        line = lines.next();

        let items = line
            .unwrap()
            .split_at(18)
            .1
            .split(", ")
            .map(|item_str| Worry::Literal(item_str.parse().unwrap()))
            .collect();
        line = lines.next();

        let operation = match line.unwrap().as_bytes()[23] {
            b'*' => Worry::mul_assign,
            b'+' => Worry::add_assign,
            _ => panic!("Unsupported operation"),
        };
        let operand = line.unwrap().split_at(25).1.parse().ok();
        line = lines.next();

        let test = line.unwrap().split_at(21).1.parse().unwrap();
        line = lines.next();

        let test_true = line.unwrap().split_at(29).1.parse().unwrap();
        line = lines.next();

        let test_false = line.unwrap().split_at(30).1.parse().unwrap();
        lines.next();

        result.push(Monkey {
            items,
            operation,
            operand,
            test,
            test_true,
            test_false,
            inspection_count: 0,
        });

        // skip empty line
        line = lines.next();
    }

    result
}

struct Monkey {
    items: VecDeque<Worry>,
    operation: for<'a> fn(&'a mut Worry, &'a Option<u32>),
    operand: Option<u32>,
    test: u32,
    test_true: usize,
    test_false: usize,

    inspection_count: u32,
}

#[derive(Debug, Clone)]
enum Worry {
    Literal(u32),
    MultiModulus(MultiModulusInt),
}

impl Worry {
    fn as_multimodulus(&self, divisors: &[u32]) -> Self {
        match self {
            Worry::Literal(val) => Self::MultiModulus(MultiModulusInt::build(divisors, *val)),
            Worry::MultiModulus(_) => self.clone(),
        }
    }
    fn mul_assign(&mut self, rhs: &Option<u32>) {
        match (self, rhs) {
            (Worry::Literal(lhs_val), Some(rhs_val)) => *lhs_val = *lhs_val * *rhs_val,
            (Worry::Literal(lhs_val), None) => *lhs_val = *lhs_val * *lhs_val,
            (Worry::MultiModulus(lhs), Some(val)) => {
                lhs.apply_literal_mul(*val);
                // *s = Worry::MultiModulus(lhs);
            }
            (Worry::MultiModulus(lhs), None) => {
                lhs.square();
                // Worry::MultiModulus(lhs)
            }
        }
    }

    fn add_assign(&mut self, rhs: &Option<u32>) {
        match (self, rhs) {
            (Worry::Literal(lhs_val), Some(rhs_val)) => *lhs_val = *lhs_val + *rhs_val,
            (Worry::Literal(lhs_val), None) => *lhs_val = *lhs_val + *lhs_val,
            (Worry::MultiModulus(lhs), Some(val)) => {
                lhs.apply_literal_add(*val);
            }
            (Worry::MultiModulus(lhs), None) => {
                lhs.double();
            }
        }
    }

    fn div_assign(&mut self, rhs: &Option<u32>) {
        match (self, rhs) {
            (Worry::Literal(lhs_val), Some(rhs_val)) => *lhs_val = *lhs_val / *rhs_val,
            (_, None) => {}
            _ => panic!("unsupported"),
        }
    }

    fn modulo(&self, rhs: u32) -> u32 {
        match self {
            Worry::Literal(v) => *v % rhs,
            Worry::MultiModulus(lhs) => lhs.modulo(rhs),
        }
    }
}

#[derive(Debug, Clone)]
struct ModulusBasedInt {
    divisor: u32,
    remainder: u32,
}

impl ModulusBasedInt {
    fn build(divisor: u32, val: u32) -> Self {
        Self {
            divisor: divisor,
            remainder: val % divisor,
        }
    }

    fn apply_literal_add(&mut self, val: u32) {
        self.remainder = self.remainder + val;
        self.normalize();
    }

    fn apply_literal_mul(&mut self, val: u32) {
        self.remainder = self.remainder * val;
        self.normalize();
    }

    fn square(&mut self) {
        self.remainder = self.remainder * self.remainder;
        self.normalize();
    }

    fn normalize(&mut self) {
        let old_remainder = self.remainder;
        self.remainder = old_remainder % self.divisor;
    }
}

#[derive(Debug, Clone)]
struct MultiModulusInt {
    vals: Vec<ModulusBasedInt>,
}

impl MultiModulusInt {
    fn build(divisors: &[u32], val: u32) -> Self {
        Self {
            vals: divisors
                .iter()
                .map(|divisor| ModulusBasedInt::build(*divisor, val))
                .collect(),
        }
    }

    fn apply_literal_add(&mut self, val: u32) {
        for v in self.vals.iter_mut() {
            v.apply_literal_add(val);
        }
    }

    fn apply_literal_mul(&mut self, val: u32) {
        for v in self.vals.iter_mut() {
            v.apply_literal_mul(val);
        }
    }

    fn square(&mut self) {
        for v in self.vals.iter_mut() {
            v.square();
        }
    }

    fn double(&mut self) {
        for v in self.vals.iter_mut() {
            v.apply_literal_mul(2);
        }
    }

    fn modulo(&self, rhs: u32) -> u32 {
        for v in self.vals.iter() {
            if v.divisor == rhs {
                return v.remainder;
            }
        }
        panic!("unsupported");
    }
}
