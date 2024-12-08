use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
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
    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (i, antenna) in parsed_input.antennas.iter().enumerate() {
        for (j, other_antenna) in parsed_input.antennas.iter().enumerate() {
            if i == j || other_antenna.frequency != antenna.frequency {
                continue;
            }

            let distance_x = other_antenna.x_pos as i32 - antenna.x_pos as i32;
            let distance_y = other_antenna.y_pos as i32 - antenna.y_pos as i32;

            let antinode_pos_x = antenna.x_pos as i32 - distance_x as i32;
            let antinode_pos_y = antenna.y_pos as i32 - distance_y as i32;

            if antinode_pos_x < 0
                || antinode_pos_x >= parsed_input.width as i32
                || antinode_pos_y < 0
                || antinode_pos_y >= parsed_input.height as i32
            {
                continue;
            }

            antinodes.insert((antinode_pos_x, antinode_pos_y));
            println!("Antinode: {antinode_pos_x}, {antinode_pos_y}");
        }
    }

    println!("Number of antinodes: {}", antinodes.len());
}

fn part_2(parsed_input: &ParsedInput) {
    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (i, antenna) in parsed_input.antennas.iter().enumerate() {
        for (j, other_antenna) in parsed_input.antennas.iter().enumerate() {
            if i == j || other_antenna.frequency != antenna.frequency {
                continue;
            }

            let distance_x = other_antenna.x_pos as i32 - antenna.x_pos as i32;
            let distance_y = other_antenna.y_pos as i32 - antenna.y_pos as i32;

            let mut next_resonance_x = antenna.x_pos as i32;
            let mut next_resonance_y = antenna.y_pos as i32;

            while next_resonance_x >= 0
                && next_resonance_x < parsed_input.width as i32
                && next_resonance_y >= 0
                && next_resonance_y < parsed_input.height as i32
            {
                antinodes.insert((next_resonance_x, next_resonance_y));
                println!(
                    "Antinode {}: {next_resonance_x}, {next_resonance_y}",
                    antenna.frequency
                );

                next_resonance_x -= distance_x;
                next_resonance_y -= distance_y;
            }
        }
    }

    println!("Number of antinodes: {}", antinodes.len());
}

struct Antenna {
    x_pos: u32,
    y_pos: u32,
    frequency: char,
}

struct ParsedInput {
    antennas: Vec<Antenna>,
    width: usize,
    height: usize,
}

fn parse_input(input: &str) -> ParsedInput {
    let mut antennas = Vec::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.trim().lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        width = line.len();
        height += 1;

        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.push(Antenna {
                    frequency: c,
                    x_pos: x as u32,
                    y_pos: y as u32,
                });
            }
        }
    }

    ParsedInput {
        antennas,
        width,
        height,
    }
}
