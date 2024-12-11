use std::time::Instant;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"125 17"#;

fn main() {
    let use_example = std::env::args().any(|v| v == "--example");

    println!("Launching, using example. {use_example}");

    let parsed = parse_input(use_example.then_some(EXAMPLE_INPUT).unwrap_or(INPUT));

    println!("part 1");
    part_1(&parsed);

    println!("=======");
    println!("part 2");
    let started = Instant::now();
    part_2(&parsed);
    let elapsed = started.elapsed();
    println!("{elapsed:?}");
}

fn part_1(parsed_input: &ParsedInput) {
    run_recursive(parsed_input, 25);
}

fn part_2(parsed_input: &ParsedInput) {
    run_recursive(parsed_input, 75);
}

fn run_recursive(input: &ParsedInput, iterations: u32) {
    let mut solver = Box::new(Solver::new(iterations as usize));

    let mut len = 0;
    for start in input {
        len += solver.find_len_recursive(*start as u64, iterations - 1);
    }

    println!("Length: {len}")
}

type ParsedInput = Vec<u32>;

fn parse_input(input: &'static str) -> ParsedInput {
    let mut output = Vec::new();

    for num_str in input.split(' ') {
        output.push(num_str.parse().unwrap());
    }

    output
}

struct Solver {
    cache: Vec<Vec<Option<u64>>>,
}

impl Solver {
    fn new(iterations: usize) -> Self {
        Self {
            cache: vec![vec![None; iterations]; 2024],
        }
    }

    fn find_len_recursive(&mut self, input: u64, remaining_depth: u32) -> u64 {
        if (input as usize) < self.cache.len() {
            if let Some(cached_result) = self.cache[input as usize][remaining_depth as usize] {
                return cached_result;
            }
        }

        let (entry_a, entry_b) = Self::step(input);
        let mut this_len = 0;

        if remaining_depth > 0 {
            this_len += self.find_len_recursive(entry_a, remaining_depth - 1);
            if let Some(entry_b) = entry_b {
                this_len += self.find_len_recursive(entry_b, remaining_depth - 1);
            }
        } else {
            this_len = 1;
            if entry_b.is_some() {
                this_len += 1;
            }
        }

        if (input as usize) < self.cache.len() {
            self.cache[input as usize][remaining_depth as usize] = Some(this_len);
        }

        this_len
    }

    fn step(entry: u64) -> (u64, Option<u64>) {
        if entry == 0 {
            return (1, None);
        }

        let s = entry.to_string();
        if s.len() % 2 == 0 {
            let half = s.len() / 2;
            let (first, second) = s.split_at(half);

            return (first.parse().unwrap(), Some(second.parse().unwrap()));
        } else {
            return (entry * 2024, None);
        }
    }
}
