const INPUT: &str = include_str!("real_input.txt");

fn main() {
    let instructions: Program = INPUT.lines().map(|s| s.parse().unwrap()).collect();

    // part 1
    {
        let mut part1 = 0;
        Machine::default().execute(&instructions, |machine| {
            if machine.cpu.clock < 221 && (machine.cpu.clock as i64 - 20) % 40 == 0 {
                part1 += machine.cpu.clock as i32 * machine.cpu.reg_x;
            }
        });
        dbg!(part1);
    }

    // part 2
    {
        println!("part 2");
        Machine::default().execute(&instructions, |machine| {
            print!("{}", machine.crt.current_pixel);
            if machine.crt.is_eol() {
                println!("");
            }
        });
    }
}

#[derive(Debug, Default)]
struct Machine {
    cpu: Cpu,
    crt: Crt,
}

impl Machine {
    fn execute<F: FnMut(&Machine)>(&mut self, program: &Program, mut observer: F) {
        // self.cpu.execute(program, |cpu| {})
        while self.cpu.program_counter < program.len() {
            self.cpu.begin_tick();
            self.crt.tick(&self.cpu);
            observer(self);
            self.cpu.end_tick(program);
        }
    }
}

#[derive(Debug)]
struct Cpu {
    reg_x: i32,
    clock: u32,

    program_counter: usize,
    delay_clocks: Option<u32>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            reg_x: 1,
            clock: 0,
            program_counter: 0,
            delay_clocks: None,
        }
    }
}

impl Cpu {
    fn begin_tick(&mut self) {
        self.clock += 1;
    }

    fn end_tick(&mut self, program: &Program) {
        match (&program[self.program_counter], self.delay_clocks) {
            (Instruction::Addx(_), None) => {
                self.delay_clocks = Some(0);
            }
            (Instruction::Addx(v), Some(0)) => {
                self.reg_x += v;
                self.program_counter += 1;
                self.delay_clocks = None;
            }
            (Instruction::Addx(_), Some(d)) => {
                self.delay_clocks = Some(d - 1);
            }
            (Instruction::Noop, _) => {
                self.program_counter += 1;
            }
        }
    }
}

#[derive(Debug, Default)]
struct Crt {
    pixel_idx: u32,
    current_pixel: char,
}

impl Crt {
    fn tick(&mut self, cpu: &Cpu) {
        let pixel_x = (self.pixel_idx % 40) as i32;
        if (pixel_x - cpu.reg_x).abs() <= 1 {
            self.current_pixel = '#'
        } else {
            self.current_pixel = '.'
        }
        self.pixel_idx += 1;
    }

    fn is_eol(&self) -> bool {
        self.pixel_idx % 40 == 0
    }
}

enum Instruction {
    Noop,
    Addx(i32),
}

impl std::str::FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..4] {
            "noop" => Ok(Self::Noop),
            "addx" => {
                let operand: i32 = s[5..].parse().map_err(|_| ())?;
                Ok(Self::Addx(operand))
            }
            _ => Err(()),
        }
    }
}

type Program = Vec<Instruction>;
