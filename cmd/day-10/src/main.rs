use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"#;

// const USING_INPUT: &str = EXAMPLE_INPUT;
const USING_INPUT: &str = INPUT;
const INPUT_WIDTH: usize = input_width(&USING_INPUT);
const INPUT_HEIGHT: usize = input_height(&USING_INPUT);

fn main() {
    // let use_example = std::env::args().any(|v| v == "--example");

    // println!("Launching, using example. {use_example}");

    // let parsed_input = parse_input(INPUT);
    dbg!(INPUT_WIDTH);
    dbg!(INPUT_HEIGHT);

    let parsed = parse_input(USING_INPUT);

    println!("part 1:");
    part_1(&parsed);

    println!("=======");
    println!("part 2");
    part_2(&parsed);
}

fn part_1(parsed_input: &ParsedInput) {
    let paths = find_paths(parsed_input);

    let mut scores: HashMap<Vec2, HashSet<Vec2>> = HashMap::new();
    for path in paths {
        if path.path.len() == 10 {
            println!("good path: {:?}", path.path);

            let set = scores.entry(path.start_position).or_default();
            set.insert(path.path.last().cloned().unwrap());
            // *(scores.entry(path.start_position).or_default()) += 1;
        }
    }
    println!("Trail heads: {}", scores.len());
    for (trail_head, set) in &scores {
        println!("Trail head: {trail_head}: {}", set.len());
    }
    let sum: usize = scores.values().map(|v| v.len()).sum();
    println!("Sum: {sum}");
}

fn part_2(parsed_input: &ParsedInput) {
    let paths = find_paths(parsed_input);

    let mut ratings: HashMap<Vec2, u32> = HashMap::new();
    for path in paths {
        if path.path.len() == 10 {
            println!("good path: {:?}", path.path);

            *(ratings.entry(path.start_position).or_default()) += 1;
        }
    }
    println!("Trail heads: {}", ratings.len());
    for (trail_head, rating) in &ratings {
        println!("Trail head: {trail_head}: {rating}");
    }
    let sum: u32 = ratings.values().sum();
    println!("Sum: {sum}");
}

#[derive(Clone, Copy, Hash, Debug, Eq, PartialEq)]
struct Vec2 {
    x: u8,
    y: u8,
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;
        self.x.fmt(f)?;
        f.write_char(',')?;
        self.y.fmt(f)?;
        f.write_char(')')
    }
}

#[derive(Clone)]
struct Path {
    start_position: Vec2,
    path: Vec<Vec2>,
    complete: bool,
}

const DIRECTIONS: [(i8, i8); 4] = [
    (1, 0),  // Right
    (0, 1),  // Down
    (-1, 0), // Left
    (0, -1), // Up
];

fn find_paths(input: &ParsedInput) -> Vec<Path> {
    let mut paths = Vec::new();

    // Find start positions
    for (y, row) in input.height_map.iter().enumerate() {
        for (x, h) in row.iter().enumerate() {
            if *h == 0 {
                paths.push(Path {
                    path: [Vec2 {
                        x: x as u8,
                        y: y as u8,
                    }]
                    .into(),
                    start_position: Vec2 {
                        x: x as u8,
                        y: y as u8,
                    },
                    complete: false,
                });
            }
        }
    }

    // do a stepping approach, recursion is a evil sin
    loop {
        let mut is_all_complete = true;

        let mut additional_paths = Vec::new();
        let mut inner_path_cache = Vec::new();

        for path in paths.iter_mut() {
            if path.complete {
                continue;
            }

            is_all_complete = false;

            let cur_path_pos = path.path[path.path.len() - 1];
            for (x_dir, y_dir) in &DIRECTIONS {
                // Out of x bounds
                if (cur_path_pos.x < 1 && *x_dir < 0)
                    || (cur_path_pos.x >= (INPUT_WIDTH - 1) as u8 && *x_dir > 0)
                {
                    continue;
                }

                // Out of y bounds
                if (cur_path_pos.y < 1 && *y_dir < 0)
                    || (cur_path_pos.y >= (INPUT_HEIGHT - 1) as u8 && *y_dir > 0)
                {
                    continue;
                }

                let new_pos_x = (cur_path_pos.x as i8 + x_dir) as u8;
                let new_pos_y = (cur_path_pos.y as i8 + y_dir) as u8;
                let new_height = input.height_map[new_pos_y as usize][new_pos_x as usize];
                let old_height = input.height_map[cur_path_pos.y as usize][cur_path_pos.x as usize];
                if new_height != old_height + 1 {
                    continue;
                }

                // Check if we traveled here before along this path
                if path
                    .path
                    .iter()
                    .any(|v| v.x == new_pos_x && v.y == new_pos_y)
                {
                    continue;
                }

                // Found next potential position
                inner_path_cache.push(Vec2 {
                    x: new_pos_x,
                    y: new_pos_y,
                });
            }

            if inner_path_cache.len() == 0 {
                path.complete = true;
            } else {
                // use the last to extend our current path
                let last = inner_path_cache.pop().unwrap();

                // split the path into new paths possibly
                //
                // obvious optimization here is instead of copying the entire path is making a reference from where the path
                // split off from
                for remaining_path in inner_path_cache.drain(..) {
                    let mut cloned = path.clone();
                    cloned.path.push(remaining_path);
                    additional_paths.push(cloned);
                }

                // finally apply extend the current path
                path.path.push(last);
            }
        }

        paths.append(&mut additional_paths);

        if is_all_complete {
            break;
        }
    }

    paths
}

struct ParsedInput {
    height_map: [[u32; INPUT_WIDTH]; INPUT_HEIGHT],
}

fn parse_input(input: &str) -> ParsedInput {
    let mut height_map = [[0u32; INPUT_WIDTH]; INPUT_HEIGHT];
    for (y, row) in input.trim().lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            height_map[y][x] = c.to_digit(10).unwrap();
        }
    }

    ParsedInput { height_map }
}

const fn input_width(s: &'static str) -> usize {
    let mut i = 0;
    let bytes = s.as_bytes().trim_ascii_start();
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            return i;
        }
        i += 1;
    }

    panic!("bad input");
}

const fn input_height(s: &'static str) -> usize {
    let mut i = 0;
    let mut newlines = 0;
    let bytes = s.as_bytes().trim_ascii_start();
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            newlines += 1;
        }
        i += 1;
    }

    newlines
}
