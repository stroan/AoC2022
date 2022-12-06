use std::collections::VecDeque;

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    // part 1
    {
        for line in INPUT.lines() {
            let part1_start = detect_message_start(line, 4);
            dbg!(part1_start);
        }
    }

    // part 2
    {
        for line in INPUT.lines() {
            let part2_start = detect_message_start(line, 14);
            dbg!(part2_start);
        }
    }
}

fn detect_message_start(s: &str, prefix_len: usize) -> usize {
    let mut window: VecDeque<u8> = Default::default();

    for (idx, b) in s.as_bytes().iter().enumerate() {
        let repeated_idx: Option<usize> = window
            .iter()
            .enumerate()
            .find(|(_, wb)| **wb == *b)
            .map(|(idx, _)| idx);
        if let Some(idx) = repeated_idx {
            for _ in 0..idx + 1 {
                window.pop_front();
            }
        }

        window.push_back(*b);

        if window.len() == prefix_len {
            return idx + 1;
        }
    }

    panic!("Not found");
}
