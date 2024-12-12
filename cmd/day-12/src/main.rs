const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
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

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn part_1(parsed_input: &ParsedInput) {
    let mut fences = 0;

    let mut region_id_gen = 0;
    let mut region_map: Vec<Vec<u32>> = Vec::new();

    for (y, row) in parsed_input.iter().enumerate() {
        region_map.push(Vec::new());
        for (x, c) in row.iter().enumerate() {
            let mut added_region = None;
            if y > 0 {
                let char_above = parsed_input[y - 1][x];
                if char_above == *c {
                    let region = region_map[y - 1][x];
                    region_map[y].push(region);
                    added_region = Some(region);
                }
            }

            if x > 0 {
                let char_above = parsed_input[y - 1][x];
                if char_above == *c {
                    if let Some(added) = added_region {
                        // MERGING TIME
                        // they started touchign tips :flush:

                        // convert all of the left to this region
                        // TODO refactor, already borrowing shit
                    } else {
                        let region = region_map[y - 1][x];
                        region_map[y].push(region);
                    }
                } else if added_region.is_none() {
                    // new regoin
                    region_map[y].push(region_id_gen);
                    region_id_gen += 1;
                }
            }

            println!("{c}");
            for neighbour in &NEIGHBOURS {
                let pos_x = x as i32 + neighbour.0;
                let pos_y = y as i32 + neighbour.1;

                if pos_x > 0
                    && (pos_x as usize) < row.len()
                    && pos_y > 0
                    && (pos_y as usize) < parsed_input.len()
                {
                    let neighbour_char = parsed_input[pos_y as usize][pos_x as usize];

                    if neighbour_char != *c {
                        fences += 1;
                    }
                }
            }
        }
    }

    println!("Fences: {fences}");
}

fn part_2(parsed_input: &ParsedInput) {}

type ParsedInput = Vec<Vec<char>>;

fn parse_input(input: &str) -> ParsedInput {
    let mut output = Vec::new();

    for line in input.trim().lines() {
        let chars = line.chars();
        output.push(chars.collect::<Vec<_>>());
    }

    output
}
