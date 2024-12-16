use std::collections::{HashMap, HashSet};

use common::IVec2;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#;

fn main() {
    let use_example = std::env::args().any(|v| v == "--example");

    println!("Launching, using example. {use_example}");

    let parsed = parse_input(use_example.then_some(EXAMPLE_INPUT).unwrap_or(INPUT));

    println!("part 1");
    // part_1(&parsed);

    println!("=======");
    println!("part 2");
    part_2(&parsed);
}

fn part_1(parsed_input: &ParsedInput) {
    let paths = parsed_input.solve_maze();
    let lowest_score_path = paths
        .iter()
        .filter(|v| v.complete)
        .min_by_key(|v| v.cost)
        .unwrap();
    println!("lowest? {}", lowest_score_path.cost);
}
fn part_2(parsed_input: &ParsedInput) {
    let paths = parsed_input.solve_maze();
    let lowest_score_path = paths
        .iter()
        .filter(|v| v.complete)
        .min_by_key(|v| v.cost)
        .unwrap();

    let mut visited_tiles = HashSet::new();

    let other_paths = paths.iter().filter(|v| v.cost == lowest_score_path.cost);

    let mut count = 0;
    for path in other_paths {
        for tile in &path.path {
            if visited_tiles.insert(tile) {
                count += 1;
            }
        }
    }

    println!(
        "lowest? {}, very nice view: {count}",
        lowest_score_path.cost
    );
}

const DIRECTIONS: [IVec2; 4] = [
    IVec2 { x: 1, y: 0 },
    IVec2 { x: 0, y: 1 },
    IVec2 { x: -1, y: 0 },
    IVec2 { x: 0, y: -1 },
];

#[derive(Debug, Clone)]
struct ParsedInput {
    start: IVec2,
    end: IVec2,
    maze: Vec<Vec<bool>>,
}

impl ParsedInput {
    fn solve_maze(&self) -> Vec<Path> {
        let mut traveled_tiles = HashMap::<IVec2, (usize, u64)>::new();
        let mut active_paths = vec![Path {
            path: vec![self.start],
            cost: 0,
            direction: 0,
            complete: false,
        }];
        traveled_tiles.insert(self.start, (0, 0));

        let mut new_paths = Vec::new();
        loop {
            let mut did_grow = false;
            for (path_i, active_path) in active_paths.iter_mut().enumerate() {
                // step 1 grow in all possible directions, creating new paths
                let mut grow_ourselves = None;

                let mut pot_dir_clockwise = active_path.direction + 1;
                if pot_dir_clockwise >= DIRECTIONS.len() {
                    pot_dir_clockwise -= DIRECTIONS.len()
                }

                let pot_dir_counter_clockwise = if active_path.direction == 0 {
                    DIRECTIONS.len() - 1
                } else {
                    active_path.direction - 1
                };

                let tail = active_path.path[active_path.path.len() - 1];
                if tail == self.end {
                    active_path.complete = true;
                    continue;
                }

                'OUTER: for grow_dir_index in [
                    active_path.direction,
                    pot_dir_clockwise,
                    pot_dir_counter_clockwise,
                ] {
                    let grow_dir = DIRECTIONS[grow_dir_index];

                    let mut this_move_cost = 1;
                    if grow_dir_index != active_path.direction {
                        this_move_cost += 1000
                    }
                    let new_cost = active_path.cost + this_move_cost;

                    let new_pos = tail + grow_dir;

                    if self.is_wall_or_oob(new_pos) {
                        continue;
                    }

                    // // check if we traveled here before
                    for pos in &active_path.path {
                        if *pos == new_pos {
                            // this path has already been down this road
                            continue 'OUTER;
                        }
                    }

                    if let Some((index, prev_cost)) = traveled_tiles.get(&new_pos) {
                        // // TODO: potentially steal
                        // if *index == path_i {
                        //     // ourselves
                        //     continue;
                        // }

                        if *prev_cost < new_cost - 1000 {
                            // our path is more expensive, don't bother
                            continue;
                        }
                    }

                    traveled_tiles.insert(new_pos, (path_i, new_cost));

                    did_grow = true;
                    if grow_ourselves.is_none() {
                        grow_ourselves = Some((new_pos, grow_dir_index, new_cost));
                    } else {
                        let mut new_path = active_path.clone();
                        new_path.cost = new_cost;
                        new_path.direction = grow_dir_index;
                        new_path.path.push(new_pos);
                        new_paths.push(new_path);
                    }
                }

                if let Some((new_pos, dir, cost)) = grow_ourselves {
                    active_path.cost = cost;
                    active_path.direction = dir;
                    active_path.path.push(new_pos);
                }
            }

            println!("found paths: {}", active_paths.len());
            if !did_grow {
                println!("done !found paths: {}", active_paths.len());
                return active_paths;
            }

            active_paths.append(&mut new_paths);
        }
    }

    fn is_wall_or_oob(&self, pos: IVec2) -> bool {
        if !self.is_in_bounds(pos) {
            return false;
        }

        self.maze[pos.y as usize][pos.x as usize]
    }

    fn is_in_bounds(&self, pos: IVec2) -> bool {
        let width = self.maze[0].len() as i64;
        let height = self.maze.len() as i64;

        pos.x >= 0 && pos.y >= 0 && pos.x < width && pos.y < height
    }
}

#[derive(Debug, Clone)]
struct Path {
    path: Vec<IVec2>,
    cost: u64,
    direction: usize,
    complete: bool,
}

fn parse_input(input: &str) -> ParsedInput {
    let mut board = Vec::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in input.trim().lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    row.push(false);
                }
                '#' => {
                    row.push(true);
                }
                'S' => {
                    row.push(false);
                    start = Some(IVec2 {
                        x: x as i64,
                        y: y as i64,
                    })
                }
                'E' => {
                    row.push(false);
                    end = Some(IVec2 {
                        x: x as i64,
                        y: y as i64,
                    })
                }
                _ => panic!("unknown dings {c}"),
            }
        }

        board.push(row);
    }

    ParsedInput {
        start: start.unwrap(),
        end: end.unwrap(),
        maze: board,
    }
}
