use std::time::Instant;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
"#;

fn main() {
    let use_example = std::env::args().any(|v| v == "--example");

    println!("Launching, using example. {use_example}");

    let parsed = parse_input(use_example.then_some(EXAMPLE_INPUT).unwrap_or(INPUT));

    println!("part 1");
    part_1(&parsed);

    let started = Instant::now();
    println!("=======");
    println!("part 2");
    let elapsed = started.elapsed();
    println!("Elapsed: {elapsed:?}");

    part_2(&parsed);
}

fn part_1(parsed_input: &ParsedInput) {
    let mut total = 0;

    for item in parsed_input {
        if let Some((a, b)) = solve_bad(item, false) {
            total += a * 3;
            total += b;
        }
    }

    println!("dingo {total}");
}

fn part_2(parsed_input: &ParsedInput) {
    let mut total = 0;

    for item in parsed_input {
        if let Some((a, b)) = solve_bad(item, true) {
            total += a * 3;
            total += b;
            // println!("Got match dingo {a}.{b}");
        } else {
            // println!("No match dingo :(");
        }
    }

    println!("dingo {total}");
}

fn solve_bad(setup: &Setup, is_part_2: bool) -> Option<(u64, u64)> {
    let mut prize = setup.prize;
    if is_part_2 {
        prize.x += 10_000_000_000_000;
        prize.y += 10_000_000_000_000;
    }

    // create the 2 line segments
    let a = FVec2 { x: 0.0, y: 0.0 };
    let b = FVec2 {
        x: setup.button_a.x as f64 * 10_000_000_000_000.0,
        y: setup.button_a.y as f64 * 10_000_000_000_000.0,
    };

    // create the 2 line segments
    let c = FVec2 {
        x: prize.x as f64,
        y: prize.y as f64,
    };
    let d = FVec2 {
        x: prize.x as f64 - (setup.button_b.x as f64 * 10_000_000_000_000.0),
        y: prize.y as f64 - (setup.button_b.y as f64 * 10_000_000_000_000.0),
    };

    let intersection = line_intersection(a, b, c, d);

    if let Some(intersection) = intersection {
        let button_a_steps = (intersection.0 / setup.button_a.x as f64) as u64;

        if let Some(solution) = test_solution(setup, prize, button_a_steps) {
            return Some(solution);
        }

        if let Some(solution) = test_solution(setup, prize, button_a_steps + 1) {
            return Some(solution);
        }

        if button_a_steps > 0 {
            if let Some(solution) = test_solution(setup, prize, button_a_steps - 1) {
                return Some(solution);
            }
        }
    }

    None
}

fn test_solution(setup: &Setup, prize: UVec2, button_a_steps: u64) -> Option<(u64, u64)> {
    let x_pos = setup.button_a.x * button_a_steps;
    let y_pos = setup.button_a.y * button_a_steps;

    if x_pos > prize.x || y_pos > prize.y {
        return None;
    }
    let distance_x = prize.x - x_pos;
    let distance_y = prize.y - y_pos;

    let remainder_x = distance_x % setup.button_b.x;
    let remainder_y = distance_y % setup.button_b.y;

    if remainder_x == 0 && remainder_y == 0 {
        let div_x = distance_x / setup.button_b.x;
        let div_y = distance_y / setup.button_b.y;

        if div_x == div_y {
            // Found solution
            return Some((button_a_steps, div_x));
        }
    }

    None
}

fn line_intersection(a: FVec2, b: FVec2, c: FVec2, d: FVec2) -> Option<(f64, f64)> {
    let top = (d.x - c.x) * (a.y - c.y) - (d.y - c.y) * (a.x - c.x);
    let bottom = (d.y - c.y) * (b.x - a.x) - (d.x - c.x) * (b.y - a.y);

    if bottom == 0.0 {
        return None;
    }

    let t = top / bottom;

    if t > 1.0 {
        return None;
    }

    // dbg!(t, b.x, b.x * t);
    let x = a.x + ((b.x - a.x) * t);
    let y = a.y + ((b.y - a.y) * t);

    return Some((x, y));
}

#[derive(Debug, Clone, Copy, Default)]
struct UVec2 {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Copy, Default)]
struct FVec2 {
    x: f64,
    y: f64,
}

impl From<UVec2> for FVec2 {
    fn from(value: UVec2) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
        }
    }
}

#[derive(Debug)]
struct Setup {
    button_a: UVec2,
    button_b: UVec2,

    prize: UVec2,
}

type ParsedInput = Vec<Setup>;

fn parse_input(input: &str) -> ParsedInput {
    let mut output = Vec::new();

    let mut iter = input.trim().lines();

    loop {
        let Some(button_a_str) = iter.next() else {
            break;
        };

        let button_b_str = iter.next().unwrap();
        let prize_str = iter.next().unwrap();

        iter.next();

        output.push(Setup {
            button_a: parse_coords(button_a_str),
            button_b: parse_coords(button_b_str),
            prize: parse_coords(prize_str),
        });
    }

    output
}

fn parse_coords(input: &str) -> UVec2 {
    let mut split = input.split(":");
    split.next();
    dbg!(input);
    let components = split.next().unwrap();

    let mut x = 0u64;
    let mut y = 0u64;

    for component_str in components.trim().split(',') {
        let trimmed = component_str.trim();
        if let Some(x_str) = trimmed.strip_prefix("X=") {
            x = x_str.parse().unwrap();
        }
        if let Some(x_str) = trimmed.strip_prefix("X+") {
            x = x_str.parse().unwrap();
        }

        if let Some(y_str) = trimmed.strip_prefix("Y=") {
            y = y_str.parse().unwrap();
        }
        if let Some(y_str) = trimmed.strip_prefix("Y+") {
            y = y_str.parse().unwrap();
        }
    }

    UVec2 { x, y }
}
