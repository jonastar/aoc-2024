use std::{convert::Infallible, str::FromStr, thread::sleep, time::Duration};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"#;

fn main() {
    let use_example = std::env::args().any(|v| v == "--example");

    println!("Launching, using example. {use_example}");

    let parsed = parse_input(use_example.then_some(EXAMPLE_INPUT).unwrap_or(INPUT));

    println!("part 1");
    part_1(&parsed);

    println!("=======");
    println!("part 2");
    part_2(&parsed);
}

fn part_1(parsed_input: &ParsedInput) {
    let mut cloned = parsed_input.clone();

    print_board(&cloned);
    println!();
    step_all_n(&mut cloned, 10000);

    let mut quadrants: [u32; 5] = [0, 0, 0, 0, 0];
    for robot in cloned {
        quadrants[get_quadrant(robot.pos)] += 1;
    }

    dbg!(&quadrants);
    let result = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    println!("Result: {result}");
}
fn part_2(parsed_input: &ParsedInput) {}

fn step_all_n(parsed_input: &mut ParsedInput, n: usize) {
    for i in 0..n {
        for robot in parsed_input.iter_mut() {
            robot.step();
        }
        println!("{i}");

        if is_christmas(&parsed_input) {
            print_board(&parsed_input);
            sleep(Duration::from_millis(500));
        }
    }
}

fn is_christmas(input: &ParsedInput) -> bool {
    for y in 0..BOARD_SIZE.y {
        'OUTER: for x in 0..BOARD_SIZE.x {
            for mod_x in -1..=1 {
                for mod_y in -1..=1 {
                    let count = count_at(
                        input,
                        IVec2 {
                            x: x + mod_x,
                            y: y + mod_y,
                        },
                    );

                    if count < 1 {
                        continue 'OUTER;
                    }
                }
            }

            return true;
        }
    }

    false
}

fn get_quadrant(pos: IVec2) -> usize {
    let center_x = BOARD_SIZE.x / 2;
    let center_y = BOARD_SIZE.y / 2;

    if pos.x == center_x || pos.y == center_y {
        return 4;
    }

    if pos.y < center_y {
        if pos.x < center_x {
            return 0;
        } else {
            return 1;
        }
    } else {
        if pos.x < center_x {
            return 2;
        } else {
            return 3;
        }
    }

    // panic!("We shouldn't be here!");
}

fn print_board(input: &ParsedInput) {
    for y in 0..BOARD_SIZE.y {
        for x in 0..BOARD_SIZE.x {
            let num_robots = count_at(input, IVec2 { x, y });
            if num_robots > 0 {
                print!("{}", num_robots.to_string());
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn count_at(input: &ParsedInput, pos: IVec2) -> usize {
    input
        .iter()
        .filter(|v| v.pos.x == pos.x && v.pos.y == pos.y)
        .count()
}

#[derive(Debug, Clone, Copy, Default)]
struct IVec2 {
    x: i64,
    y: i64,
}

impl FromStr for IVec2 {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        dbg!(s);
        let mut split = s.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();

        Ok(Self { x, y })
    }
}

// const BOARD_SIZE: IVec2 = IVec2 { x: 11, y: 7 };
const BOARD_SIZE: IVec2 = IVec2 { x: 101, y: 103 };

#[derive(Debug, Clone, Copy, Default)]
struct Robot {
    pos: IVec2,
    vel: IVec2,
}

impl Robot {
    fn step(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        self.pos.x = self.pos.x % (BOARD_SIZE.x);
        self.pos.y = self.pos.y % (BOARD_SIZE.y);

        if self.pos.x < 0 {
            self.pos.x += BOARD_SIZE.x;
        }
        if self.pos.y < 0 {
            self.pos.y += BOARD_SIZE.y;
        }
    }
}

impl FromStr for Robot {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // p=0,4 v=3,-3
        let mut split = s.split(' ');
        let p_part: IVec2 = split
            .next()
            .unwrap()
            .trim()
            .trim_start_matches("p=")
            .parse()
            .unwrap();
        let v_part: IVec2 = split
            .next()
            .unwrap()
            .trim()
            .trim_start_matches("v=")
            .parse()
            .unwrap();

        Ok(Self {
            pos: p_part,
            vel: v_part,
        })
    }
}

type ParsedInput = Vec<Robot>;

fn parse_input(input: &str) -> ParsedInput {
    input.trim().lines().map(|v| v.parse().unwrap()).collect()
}
