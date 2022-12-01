const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let lines: Vec<&str> = INPUT.split('\n').collect();

    let elves: Vec<&[&str]> = lines.split(|l| *l == "").collect();
    let summed_elves: Vec<i32> = elves
        .iter()
        .map(|elf| {
            elf.iter()
                .fold(0, |acc, cal_str| acc + cal_str.parse::<i32>().unwrap())
        })
        .collect();

    // part 1
    {
        let biggest = summed_elves.iter().max().unwrap();
        dbg!(biggest);
    }

    // part 2
    {
        let sorted = {
            let mut c = summed_elves.clone();
            c.sort();
            c
        };
        let biggest_3 = &sorted[(sorted.len() - 3)..];
        let biggest_3_sum = biggest_3.iter().fold(0, |acc, x| acc + x);
        dbg!(biggest_3_sum);
    }
}
