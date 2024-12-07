use std::str::FromStr;

const INPUT: &str = include_str!("input.txt");

const EXAMPLE_INPUT: &str = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
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
    let mut sum = 0;
    for equation in &parsed_input.equations {
        if equation.is_part1_true() {
            // println!("{equation:?} is true!");
            sum += equation.equation;
        }
    }
    println!("Sum {sum}");
}

fn part_2(parsed_input: &ParsedInput) {
    let mut sum = 0;
    for equation in &parsed_input.equations {
        if equation.is_part2_true() {
            // println!("{equation:?} is true!");
            sum += equation.equation;
        }
    }
    println!("Sum {sum}");
}

#[derive(Debug)]
struct Equation {
    equation: i128,
    // number and the number of base 10 digits it has
    numbers: Vec<(i128, u32)>,
}

impl Equation {
    fn is_part1_true(&self) -> bool {
        let mut last_frame: Vec<i128> = Vec::new();
        for (number, _) in &self.numbers {
            if last_frame.is_empty() {
                last_frame = vec![*number];
                continue;
            }

            let mut next_frame = Vec::new();

            for last in &last_frame {
                next_frame.push(last + number);
                next_frame.push(last * number);
            }
            last_frame = next_frame;
        }

        // dbg!(&last_frame);
        last_frame.iter().any(|v| *v == self.equation)
    }

    fn is_part2_true(&self) -> bool {
        let mut last_frame: Vec<i128> = Vec::new();
        for (number, num_digits) in &self.numbers {
            if last_frame.is_empty() {
                last_frame = vec![*number];
                continue;
            }

            let mut next_frame = Vec::new();

            for last in &last_frame {
                next_frame.push(last + number);
                next_frame.push(last * number);

                let concatenated = (last * (10i128.pow(*num_digits))) + number;
                next_frame.push(concatenated);
                // println!("{last} || {number} = {concatenated}");
            }
            last_frame = next_frame;
        }

        // dbg!(&last_frame);
        last_frame.iter().any(|v| *v == self.equation)
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut equation_split = s.split(":");
        // dbg!(s);
        let equation_num = equation_split.next().unwrap().parse().unwrap();

        let numbers_str = equation_split.next().unwrap().trim();
        let numbers = numbers_str
            .split(" ")
            .map(|v| (v.parse::<i128>().unwrap(), v.len() as u32))
            .collect::<Vec<_>>();

        Ok(Self {
            equation: equation_num,
            numbers,
        })
    }
}

struct ParsedInput {
    equations: Vec<Equation>,
}

fn parse_input(input: &str) -> ParsedInput {
    let equations = input
        .trim()
        .lines()
        .filter(|v| !v.is_empty())
        .map(|v| v.parse().unwrap())
        .collect::<Vec<Equation>>();
    ParsedInput { equations }
}
