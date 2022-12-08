use std::fmt::Debug;

const INPUT: &str = include_str!("real_input.txt");

fn main() {
    // part 1
    {
        let part1 = treeval(
            true,
            |acc, self_h, other_h| *acc && other_h < self_h,
            |d1, d2, d3, d4| (d1 || d2 || d3 || d4) as i32,
            0,
            |acc, t| *acc + t,
        );
        dbg!(part1);
    }

    // part 2
    {
        let part2 = treeval(
            (0, true),
            |(acc_score, cont), self_h, other_h| {
                if !cont {
                    (*acc_score, *cont)
                } else {
                    (acc_score + 1, other_h < self_h)
                }
            },
            |d1, d2, d3, d4| d1.0 * d2.0 * d3.0 * d4.0,
            0,
            |acc, t| {
                if t > *acc {
                    t
                } else {
                    *acc
                }
            },
        );
        dbg!(part2);
    }
}

fn treeval<
    R1: Clone + Debug,
    R2: Debug,
    R3: Debug,
    F1: Fn(&R1, u8, u8) -> R1,
    F2: Fn(R1, R1, R1, R1) -> R2,
    F3: Fn(&R3, R2) -> R3,
>(
    dir_acc: R1,
    dir: F1,
    tree_score: F2,
    result_acc: R3,
    result_fold: F3,
) -> R3 {
    let mut rows: Vec<Vec<u8>> = vec![];
    for line in INPUT.lines() {
        rows.push(line.bytes().map(|b| b - b'0').collect());
    }
    let width = rows[0].len();
    let height = rows.len();

    let mut result = result_acc;
    for y in 0..height {
        for x in 0..width {
            let h = rows[y][x];
            let mut neg_x = dir_acc.clone();
            for i in (0..x).rev() {
                neg_x = dir(&neg_x, h, rows[y][i]);
            }

            let mut pos_x = dir_acc.clone();
            for i in (x + 1)..width {
                pos_x = dir(&pos_x, h, rows[y][i]);
            }

            let mut neg_y = dir_acc.clone();
            for i in (0..y).rev() {
                neg_y = dir(&neg_y, h, rows[i][x]);
            }

            let mut pos_y = dir_acc.clone();
            for i in (y + 1)..height {
                pos_y = dir(&pos_y, h, rows[i][x]);
            }

            let score = tree_score(neg_x, pos_x, neg_y, pos_y);
            result = result_fold(&result, score);
        }
    }

    result
}
