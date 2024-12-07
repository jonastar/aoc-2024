const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
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

fn part_1(parsed_input: &ParsedInput) {}
fn part_2(parsed_input: &ParsedInput) {}

struct ParsedInput {}

fn parse_input(input: &str) -> ParsedInput {
    todo!();
}
