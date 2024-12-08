use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut parsed_input = MapState::parse_from_input();

    println!("part 1");
    let started = Instant::now();
    part_1(&mut parsed_input);
    let elapsed = started.elapsed();
    println!("{elapsed:?}");

    println!("=======");
    println!("part 2");
    let started = Instant::now();
    part_2(parsed_input);
    let elapsed = started.elapsed();
    println!("{elapsed:?}");
}

fn part_1(map: &mut MapState) {
    loop {
        let (is_in_bounds, _) = map.tick();
        if !is_in_bounds {
            break;
        }
    }

    let unique_steps: usize = map
        .visited_tiles
        .iter()
        .map(|v| v.iter().filter(|t| **t != 0).count())
        .sum();

    println!("Unique steps: {unique_steps}");
}

fn part_2(mut initial_map: MapState) {
    let mut loop_obstacles = 0;

    initial_map.reset();
    initial_map.tick_until_out_of_bounds_or_loop();
    let mut check_coords = Vec::new();
    // let check_coords = initial_map
    //     .visited_tiles
    //     .keys()
    //     .cloned()
    //     .collect::<Vec<_>>();
    for (y, row) in initial_map.visited_tiles.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell > 0 {
                check_coords.push((x, y));
            }
        }
    }

    initial_map.reset();

    for (x, y) in check_coords {
        if initial_map.tiles[y][x].is_obstacle
            || (initial_map.guard_pos.0 == x as i32 && initial_map.guard_pos.1 == y as i32)
        {
            continue;
        }

        // let mut cloned = initial_map.clone();
        initial_map.tiles[y][x].is_obstacle = true;

        if SimulCompleteResult::Loop == initial_map.tick_until_out_of_bounds_or_loop() {
            loop_obstacles += 1;
            // println!("Found loop obstacle on {x}, {y}");
        }

        initial_map.tiles[y][x].is_obstacle = false;
        initial_map.reset();
    }

    println!("Total loop obstacles: {loop_obstacles}");
}

const DIRECTIONS: [(i32, i32); 4] = [
    // up
    (0, -1),
    // right
    (1, 0),
    // down
    (0, 1),
    // left
    (-1, 0),
];
const VISITED_DIRECTION_MASKS: [u8; 4] = [1 << 1, 1 << 3, 1 << 5, 1 << 7];

#[derive(Clone)]
struct TileState {
    is_obstacle: bool,
}

#[derive(Clone)]
struct MapState {
    start_guard_pos: (i32, i32),
    tiles: Vec<Vec<TileState>>,
    map_width: usize,
    guard_pos: (i32, i32),
    guard_dir: usize,
    visited_tiles: Vec<Vec<u8>>,
}

#[derive(PartialEq)]
enum SimulCompleteResult {
    OutOfBounds,
    Loop,
}

impl MapState {
    fn parse_from_input() -> Self {
        let mut tiles = Vec::new();
        let mut visited_map = Vec::new();
        let mut guard_pos = None;
        let mut width = 0;

        for (y, line) in INPUT.lines().enumerate() {
            if width == 0 {
                width = line.len()
            }

            let line_tiles = line
                .chars()
                .map(|v| TileState {
                    is_obstacle: v == '#',
                })
                .collect::<Vec<_>>();
            tiles.push(line_tiles);

            let mut visited = Vec::new();
            for _ in 0..line.len() {
                visited.push(0);
            }
            visited_map.push(visited);

            if guard_pos.is_none() {
                if let Some((x_pos, _)) = line.chars().enumerate().find(|(_, v)| *v == '^') {
                    guard_pos = Some((x_pos as i32, y as i32));
                }
            }
        }

        assert!(width != 0);

        // init starting tile state
        // we could probs do the tracking 1 tile behind to avoid this but whatever
        let guard_pos = guard_pos.unwrap();
        visited_map[guard_pos.1 as usize][guard_pos.0 as usize] = 1;

        Self {
            guard_pos: guard_pos,
            start_guard_pos: guard_pos,
            // facing up
            guard_dir: 0,
            tiles,
            map_width: width,

            visited_tiles: visited_map,
        }
    }

    // 1st in bounds, second looping
    #[inline]
    fn tick(&mut self) -> (bool, bool) {
        let dir = DIRECTIONS[self.guard_dir];
        let next_pos_x = self.guard_pos.0 + dir.0;
        let next_pos_y = self.guard_pos.1 + dir.1;

        if !self.is_in_bounds(next_pos_x, next_pos_y) {
            self.guard_pos = (next_pos_x, next_pos_y);
            return (false, false);
        }

        let next_tile = &mut self.tiles[next_pos_y as usize][next_pos_x as usize];
        if next_tile.is_obstacle {
            // an obstacle was hit R O T A T E
            self.rotate_guard();
            return (true, false);
        }

        // dbg!(next_pos_x, next_pos_y);
        self.guard_pos = (next_pos_x, next_pos_y);

        // mark as visited
        self.visited_tiles[next_pos_y as usize][next_pos_x as usize] += 1 << (self.guard_dir * 2);

        let is_looping = self.visited_tiles[next_pos_y as usize][next_pos_x as usize]
            & VISITED_DIRECTION_MASKS[self.guard_dir]
            != 0;

        (true, is_looping)
    }

    fn tick_until_out_of_bounds_or_loop(&mut self) -> SimulCompleteResult {
        loop {
            let (is_in_bounds, is_looping) = self.tick();

            if is_in_bounds && is_looping {
                // dbg!(self.guard_pos);
                return SimulCompleteResult::Loop;
            } else if !is_in_bounds {
                return SimulCompleteResult::OutOfBounds;
            }
        }
    }

    fn rotate_guard(&mut self) {
        self.guard_dir += 1;
        if self.guard_dir >= DIRECTIONS.len() {
            self.guard_dir = 0;
        }
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.map_width as i32 || y >= self.tiles.len() as i32 {
            return false;
        }

        true
    }

    fn reset(&mut self) {
        for row in &mut self.visited_tiles {
            row.fill(0);
        }

        self.visited_tiles[self.start_guard_pos.1 as usize][self.start_guard_pos.0 as usize] = 1;
        self.guard_pos = self.start_guard_pos;
        self.guard_dir = 0;
    }
}
