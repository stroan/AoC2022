use std::array::IntoIter;

const INPUT: &str = include_str!("real_input.txt");
type ItemCounts = [u32; 52];

fn main() {
    // part 1
    {
        let part1 = score_overlaps(INPUT, |input| {
            input
                .lines()
                .map(|line| line.as_bytes().chunks(line.len() / 2))
        });
        dbg!(part1);
    }

    // part 2
    {
        let part2 = score_overlaps(INPUT, |input| GroupIter {
            inner: input.lines().map(|line| line.as_bytes()),
        });
        dbg!(part2);
    }
}

fn prioritize(item: u8) -> usize {
    (if item >= b'a' {
        item - b'a'
    } else {
        26 + item - b'A'
    }) as usize
}

fn score_overlaps<
    'a,
    OuterIter: Iterator<Item = InnerIter>,
    InnerIter: Iterator<Item = &'a [u8]>,
    F: Fn(&'a str) -> OuterIter,
>(
    input: &'a str,
    grouper: F,
) -> u32 {
    let mut sum = 0;
    for outer in grouper(input) {
        let mut counts = [0; 52];
        for inner in outer {
            register_items(inner, &mut counts);
        }
        sum += most_common_priority(&mut counts);
    }
    sum
}

fn register_items(items: &[u8], counts: &mut ItemCounts) {
    let mut seen = [false; 52];
    for item in items.iter() {
        let priority = prioritize(*item) as usize;
        if !seen[priority] {
            counts[priority] += 1;
            seen[priority] = true;
        }
    }
}

fn most_common_priority(counts: &mut ItemCounts) -> u32 {
    (counts
        .iter()
        .enumerate()
        .max_by_key(|(_idx, count)| **count)
        .unwrap()
        .0
        + 1) as u32
}

struct GroupIter<T: Iterator<Item = &'static [u8]> + Clone> {
    inner: T,
}

impl<T: Iterator<Item = &'static [u8]> + Clone> Iterator for GroupIter<T> {
    type Item = IntoIter<&'static [u8], 3>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|first| {
            [
                first,
                self.inner.next().unwrap(),
                self.inner.next().unwrap(),
            ]
            .into_iter()
        })
    }
}
