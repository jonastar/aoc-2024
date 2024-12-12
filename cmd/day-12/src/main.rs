use std::{collections::HashSet, ops::ControlFlow};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
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

const NEIGHBORS: [(i32, i32); 4] = [
    // aa
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
];

const EDGE_RIGHT: usize = 0;
const EDGE_DOWN: usize = 1;
const EDGE_LEFT: usize = 2;
const EDGE_UP: usize = 3;

fn part_1(parsed_input: &ParsedInput) {
    let (region_map, highest_id) = find_regions(parsed_input);

    let mut sum = 0;

    for id in 0..highest_id {
        let mut area = 0;
        let mut fences = 0;

        let height = parsed_input.len();
        let width = parsed_input[0].len();

        // let mut c = None;

        for y in 0..height {
            for x in 0..width {
                if region_map[y][x] == id {
                    // println!("{}", id);
                    area += 1;

                    for (dir_x, dir_y) in &NEIGHBORS {
                        let new_pos_x = x as i32 + dir_x;
                        let new_pos_y = y as i32 + dir_y;

                        if !is_region_checked(&region_map, new_pos_x, new_pos_y, id) {
                            fences += 1;
                        }
                    }
                }
            }
        }

        let this_sum = area * fences;
        sum += this_sum;
        println!("{area}a * {fences}f = {this_sum}")
    }

    println!("sum: {sum}");
}

fn part_2(parsed_input: &ParsedInput) {
    let (region_map, highest_id) = find_regions(parsed_input);

    let mut sum = 0;

    for id in 0..highest_id {
        let mut area = 0;
        let mut fences = 0;

        let height = parsed_input.len();
        let width = parsed_input[0].len();

        let mut skip_map = HashSet::new();

        for y in 0..height {
            for x in 0..width {
                if region_map[y][x] == id {
                    // println!("{}", id);
                    area += 1;

                    let edges = get_edge_directions(&region_map, x as u32, y as u32, id);
                    if edges[EDGE_UP] {
                        if !skip_map.contains(&(x, y, EDGE_UP)) {
                            extend_fence_right(
                                x,
                                y,
                                width,
                                id,
                                &region_map,
                                &mut skip_map,
                                EDGE_UP,
                            );

                            fences += 1;
                        }
                    }

                    if edges[EDGE_DOWN] {
                        if !skip_map.contains(&(x, y, EDGE_DOWN)) {
                            extend_fence_right(
                                x,
                                y,
                                width,
                                id,
                                &region_map,
                                &mut skip_map,
                                EDGE_DOWN,
                            );

                            fences += 1;
                        }
                    }

                    if edges[EDGE_LEFT] {
                        if !skip_map.contains(&(x, y, EDGE_LEFT)) {
                            extend_fence_down(
                                x,
                                y,
                                height,
                                id,
                                &region_map,
                                &mut skip_map,
                                EDGE_LEFT,
                            );

                            fences += 1;
                        }
                    }

                    if edges[EDGE_RIGHT] {
                        if !skip_map.contains(&(x, y, EDGE_RIGHT)) {
                            extend_fence_down(
                                x,
                                y,
                                height,
                                id,
                                &region_map,
                                &mut skip_map,
                                EDGE_RIGHT,
                            );

                            fences += 1;
                        }
                    }
                }
            }
        }

        let this_sum = area * fences;
        sum += this_sum;
        println!("{id}: {area}a * {fences}f = {this_sum}")
    }

    println!("sum: {sum}");
}

fn extend_fence_right(
    x: usize,
    y: usize,
    width: usize,
    region_id: u32,
    region_map: &Vec<Vec<u32>>,
    skip_map: &mut HashSet<(usize, usize, usize)>,
    edge: usize,
) {
    for inner_x in x..width {
        if let ControlFlow::Break(_) =
            extend_step(skip_map, inner_x, y, edge, region_map, region_id)
        {
            break;
        }
        println!("{region_id}: Extended fence right {inner_x}.{y}")
    }
}

fn extend_fence_down(
    x: usize,
    y: usize,
    height: usize,
    region_id: u32,
    region_map: &Vec<Vec<u32>>,
    skip_map: &mut HashSet<(usize, usize, usize)>,
    edge: usize,
) {
    for inner_y in y..height {
        if let ControlFlow::Break(_) =
            extend_step(skip_map, x, inner_y, edge, region_map, region_id)
        {
            break;
        }

        println!("{region_id}: Extended fence down {x}.{inner_y}")
    }
}

fn extend_step(
    skip_map: &mut HashSet<(usize, usize, usize)>,
    x: usize,
    y: usize,
    edge: usize,
    region_map: &Vec<Vec<u32>>,
    region_id: u32,
) -> ControlFlow<()> {
    if skip_map.contains(&(x, y, edge)) {
        return ControlFlow::Break(());
    }

    if !is_region_checked(region_map, x as i32, y as i32, region_id) {
        return ControlFlow::Break(());
    }

    let inner_edges = get_edge_directions(region_map, x as u32, y as u32, region_id);
    if inner_edges[edge] {
        skip_map.insert((x, y, edge));
    } else {
        return ControlFlow::Break(());
    }

    ControlFlow::Continue(())
}

fn get_edge_directions(region_map: &Vec<Vec<u32>>, x: u32, y: u32, region: u32) -> [bool; 4] {
    let mut edges = [false; 4];

    for (i, (dir_x, dir_y)) in NEIGHBORS.iter().enumerate() {
        let new_pos_x = x as i32 + dir_x;
        let new_pos_y = y as i32 + dir_y;

        if !is_region_checked(&region_map, new_pos_x, new_pos_y, region) {
            edges[i] = true;
        }
    }

    edges
}

fn is_region_checked(region_map: &Vec<Vec<u32>>, x: i32, y: i32, region: u32) -> bool {
    let height = region_map.len();
    let width = region_map[0].len();

    if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
        return false;
    }

    region_map[y as usize][x as usize] == region
}

fn find_regions(parsed_input: &ParsedInput) -> (Vec<Vec<u32>>, u32) {
    let mut region_id_gen = 0;
    let mut region_map: Vec<Vec<u32>> = Vec::new();

    let height = parsed_input.len();
    let width = parsed_input[0].len();

    for y in 0..height {
        region_map.push(Vec::new());

        for x in 0..width {
            let c = parsed_input[y][x];

            let mut added_region = None;
            if y > 0 {
                let char_above = parsed_input[y - 1][x];
                if char_above == c {
                    let region = region_map[y - 1][x];
                    region_map[y].push(region);
                    added_region = Some(region);
                }
            }

            if x > 0 {
                let char_before = parsed_input[y][x - 1];
                if char_before == c {
                    if let Some(added) = added_region {
                        // MERGING TIME
                        // they started touching tips :flush:
                        let region_here = region_map[y][x - 1];
                        merge_regions(&mut region_map, added, region_here);
                        added_region = Some(added);
                    } else {
                        let region = region_map[y][x - 1];
                        region_map[y].push(region);
                        added_region = Some(region);
                    }
                }
            }

            if added_region.is_none() {
                // new region
                region_map[y].push(region_id_gen);
                region_id_gen += 1;
            }
        }
    }

    for row in &region_map {
        for region in row {
            print!("{:3}", region)
        }
        println!("");
    }

    (region_map, region_id_gen)
}

// Merges region_b into region_a
fn merge_regions(region_map: &mut Vec<Vec<u32>>, region_a: u32, region_b: u32) {
    let height = region_map.len();
    let width = region_map[0].len();

    for y in 0..height {
        for x in 0..width {
            if x >= region_map[y].len() {
                // We are done
                return;
            }

            if region_map[y][x] == region_b {
                region_map[y][x] = region_a;
            }
        }
    }
}

type ParsedInput = Vec<Vec<char>>;

fn parse_input(input: &str) -> ParsedInput {
    let mut output = Vec::new();

    for line in input.trim().lines() {
        let chars = line.chars();
        output.push(chars.collect::<Vec<_>>());
    }

    output
}
