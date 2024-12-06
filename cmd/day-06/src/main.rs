use std::thread;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let parsed_input = MapState::parse_from_input();

    println!("part 1");
    part_1(parsed_input.clone());

    println!("=======");
    println!("part 2");
    part_2(parsed_input);
}

fn part_1(mut map: MapState) {
    let mut steps = 0;
    while map.is_guard_in_bounds() {
        if map.tick() {
            steps += 1;
        } else {
            println!("R O T A T E");
        }

        // map.print_board();

        // if steps > 5{
        //     break;
        // }
    }

    let unique_steps: i32 = map
        .tiles
        .iter()
        .map(|v| v.iter().filter(|t| t.is_visited()).count() as i32)
        .sum();

    println!("Total steps: {steps}, unqiue: {unique_steps}");
}

fn part_2(initial_map: MapState) {
    thread::scope(|scope| {
        let mut loop_obstacles = 0;

        for (y, row) in initial_map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.is_obstacle
                    || (initial_map.guard_pos.0 == x as i32
                        && initial_map.guard_pos.1 == y as i32)
                {
                    continue;
                }

                let mut cloned = initial_map.clone();
                cloned.tiles[y][x].is_obstacle = true;

                println!("Simulating obstacle on {x}, {y}");
                if SimulCompleteResult::Loop == cloned.tick_until_out_of_bounds_or_loop() {
                    loop_obstacles += 1;
                    println!("Found loop obstacle on {x}, {y}");
                }
            }
        }

        println!("Total loop obstacles: {loop_obstacles}");
    });
}

fn part_2_mt(initial_map: MapState) {
    thread::scope(|scope| {
        let mut handles = Vec::new();

        for (y, row) in initial_map.tiles.iter().enumerate() {
            let cloned_y = y;
            let thread_inital = initial_map.clone();
            let handle = scope.spawn(move || {
                let mut loop_obstacles_inner = 0;
                for (x, tile) in row.iter().enumerate() {
                    if tile.is_obstacle
                        || (&initial_map.guard_pos.0 == &(x as i32)
                            && &initial_map.guard_pos.1 == &(cloned_y as i32))
                    {
                        continue;
                    }

                    let mut cloned = thread_inital.clone();
                    cloned.tiles[y][x].is_obstacle = true;

                    println!("Simulating obstacle on {x}, {y}");
                    if SimulCompleteResult::Loop == cloned.tick_until_out_of_bounds_or_loop() {
                        loop_obstacles_inner += 1;
                        println!("Found loop obstacle on {x}, {y}");
                    }
                }

                return loop_obstacles_inner;
            });

            handles.push(handle);
        }

        let total: i32 = handles.into_iter().map(|v| v.join().unwrap()).sum();
        println!("Total loop obstacles: {total}");
    });
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

#[derive(Clone)]
struct TileState {
    is_obstacle: bool,
    visited_directions: [i32; 4],
}

impl TileState {
    fn is_visited(&self) -> bool {
        self.visited_directions.iter().any(|v| *v > 0)
    }
}

#[derive(Clone)]
struct MapState {
    tiles: Vec<Vec<TileState>>,
    map_width: usize,
    guard_pos: (i32, i32),
    guard_dir: usize,
}

#[derive(PartialEq)]
enum SimulCompleteResult {
    OutOfBound,
    Loop,
}

impl MapState {
    fn parse_from_input() -> Self {
        let mut tiles = Vec::new();
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
                    visited_directions: [0, 0, 0, 0],
                })
                .collect::<Vec<_>>();
            tiles.push(line_tiles);

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
        tiles[guard_pos.1 as usize][guard_pos.0 as usize].visited_directions[0] = 1;

        Self {
            guard_pos: guard_pos,
            // facing up
            guard_dir: 0,
            tiles,
            map_width: width,
        }
    }

    fn tick(&mut self) -> bool {
        let dir = DIRECTIONS[self.guard_dir];
        let next_pos_x = self.guard_pos.0 + dir.0;
        let next_pos_y = self.guard_pos.1 + dir.1;

        if !self.is_in_bounds(next_pos_x, next_pos_y) {
            self.guard_pos = (next_pos_x, next_pos_y);
            return true;
        }

        if !self.is_obstacle(next_pos_x, next_pos_y) {
            // dbg!(next_pos_x, next_pos_y);
            self.guard_pos = (next_pos_x, next_pos_y);
            // mark as visited

            self.tiles[next_pos_y as usize][next_pos_x as usize].visited_directions
                [self.guard_dir] += 1;

            // dbg!(self.tiles[next_pos_y as usize][next_pos_x as usize].visited_directions);

            return true;
        }

        // an obstacle was hit R O T A T E
        self.rotate_guard();
        false
    }

    fn tick_until_out_of_bounds_or_loop(&mut self) -> SimulCompleteResult {
        while self.is_guard_in_bounds() {
            self.tick();
            // dbg!(self.guard_pos);
            if self.is_loop() {
                return SimulCompleteResult::Loop;
            }
        }

        SimulCompleteResult::OutOfBound
    }

    fn is_loop(&self) -> bool {
        // if we visited the same tile with the same direction more than once were in a loop
        if self.is_guard_in_bounds() {
            let tile = &self.tiles[self.guard_pos.1 as usize][self.guard_pos.0 as usize];
            if tile.visited_directions.iter().any(|v| *v > 1) {
                return true;
            }
        }

        false
    }

    fn rotate_guard(&mut self) {
        self.guard_dir += 1;
        if self.guard_dir >= DIRECTIONS.len() {
            self.guard_dir = 0;
        }
    }

    // assumes x and y is in bounds
    fn is_obstacle(&self, x: i32, y: i32) -> bool {
        let tile = &self.tiles[y as usize][x as usize];
        tile.is_obstacle
    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        if x < 0 || y < 0 || x >= self.map_width as i32 || y >= self.tiles.len() as i32 {
            return false;
        }

        true
    }

    fn is_guard_in_bounds(&self) -> bool {
        self.is_in_bounds(self.guard_pos.0, self.guard_pos.1)
    }

    fn print_board(&self) {
        println!("------");
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell.is_obstacle {
                    print!("#")
                } else if self.guard_pos.0 == x as i32 && self.guard_pos.1 == y as i32 {
                    print!("*")
                } else if cell.is_visited() {
                    print!("X")
                } else {
                    print!(".")
                }
            }
            print!("\n");
        }
    }
}
