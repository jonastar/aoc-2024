use common::IVec2;

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
"#;
// <vv<<^^<<^^

fn main() {
    let use_example = std::env::args().any(|v| v == "--example");

    println!("Launching, using example. {use_example}");

    let parsed_p1 = parse_input(use_example.then_some(EXAMPLE_INPUT).unwrap_or(INPUT), false);

    println!("part 1");
    part_1(parsed_p1);

    let parsed_p2 = parse_input(use_example.then_some(EXAMPLE_INPUT).unwrap_or(INPUT), true);
    println!("=======");
    println!("part 2");
    part_2(parsed_p2);
}

fn part_1(mut parsed_input: ParsedInput) {
    parsed_input.state.step_all(&parsed_input.moves);
    let coords = parsed_input.state.all_box_coords();
    let sum: i64 = coords.iter().map(|v| (v.y * 100) + v.x).sum();
    println!("Sum: {sum}");
}

fn part_2(mut parsed_input: ParsedInput) {
    parsed_input.state.print();

    parsed_input.state.step_all(&parsed_input.moves);
    let coords = parsed_input.state.all_box_coords();
    let sum: i64 = coords.iter().map(|v| (v.y * 100) + v.x).sum();
    println!("Sum: {sum}");
}

#[derive(Debug, Clone, Copy)]
enum Spot {
    Free,
    Wall,
    // Robot,
    Box,
    BoxL,
    BoxR,
}

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Left,
    Down,
    Right,
}

impl Move {
    fn ivec_dir(&self) -> IVec2 {
        match self {
            Move::Up => IVec2 { x: 0, y: -1 },
            Move::Left => IVec2 { x: -1, y: 0 },
            Move::Down => IVec2 { x: 0, y: 1 },
            Move::Right => IVec2 { x: 1, y: 0 },
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    board: Board,
    robot_pos: IVec2,
}

type Board = Vec<Vec<Spot>>;

impl State {
    fn dims(&self) -> IVec2 {
        IVec2 {
            x: self.board[0].len() as i64,
            y: self.board.len() as i64,
        }
    }

    fn try_push_box(&mut self, pos: IVec2, dir: IVec2) -> bool {
        let mut needs_to_follow = self.get_push_follow(pos).unwrap();
        let mut leaf = needs_to_follow.clone();

        // do a first pass and collect all the tiles that need to move, returning if we hit a wall
        loop {
            let mut new_leaf = Vec::new();
            // attempt to move the leaf
            for item in &leaf {
                let spot = self.board[item.y as usize][item.x as usize];
                let should_check = match spot {
                    Spot::Free => unreachable!(),
                    Spot::Wall => unreachable!(),
                    Spot::Box => true,
                    Spot::BoxL => {
                        // check only if going left or up/down
                        if dir.y != 0 || dir.x < 0 {
                            true
                        } else {
                            false
                        }
                    }
                    Spot::BoxR => {
                        // check only if going right or up/down
                        if dir.y != 0 || dir.x > 0 {
                            true
                        } else {
                            false
                        }
                    }
                };
                // dbg!(should_check, spot, dir,);
                if !should_check {
                    continue;
                }

                let check_pos = *item + dir;
                let Some(inner_new_leafs) = self.get_push_follow(check_pos) else {
                    return false; // hit wall
                };

                new_leaf.extend_from_slice(&inner_new_leafs);
                needs_to_follow.extend_from_slice(&inner_new_leafs);
            }

            // dbg!(&new_leaf);

            leaf = new_leaf;

            if leaf.len() < 1 {
                // done
                break;
            }
        }

        let spot_types = needs_to_follow
            .iter()
            .map(|v| self.board[v.y as usize][v.x as usize])
            .collect::<Vec<_>>();

        // first pass, replace them all with free spots,
        // doing it in multiple passes as otherwise shits order dependent on the direction
        for pos in &needs_to_follow {
            self.board[pos.y as usize][pos.x as usize] = Spot::Free;
        }

        // assign the moved dingses
        for (i, pos) in needs_to_follow.iter().enumerate() {
            let new_pos = *pos + dir;
            self.board[new_pos.y as usize][new_pos.x as usize] = spot_types[i];
        }

        true
    }

    fn get_push_follow(&self, pos: IVec2) -> Option<Vec<IVec2>> {
        let spot = self.board[pos.y as usize][pos.x as usize];

        match spot {
            Spot::Free => Some(Vec::new()),
            Spot::Wall => None,
            Spot::Box => Some(vec![pos]),
            Spot::BoxL => Some(vec![
                pos,
                IVec2 {
                    x: pos.x + 1,
                    y: pos.y,
                },
            ]),
            Spot::BoxR => Some(vec![
                pos,
                IVec2 {
                    x: pos.x - 1,
                    y: pos.y,
                },
            ]),
        }
    }

    fn is_in_bounds(&self, pos: IVec2) -> bool {
        let dims = self.dims();

        if pos.x < 0 || pos.y < 0 || pos.x >= dims.x || pos.y >= dims.y {
            return false;
        }

        true
    }

    fn move_robot(&mut self, movement: Move) -> bool {
        let new_pos = self.robot_pos + movement.ivec_dir();
        if !self.is_in_bounds(new_pos) {
            return false;
        }

        let spot = self.board[new_pos.y as usize][new_pos.x as usize];
        match spot {
            Spot::Free => {}
            Spot::Box | Spot::BoxL | Spot::BoxR => {
                if !self.try_push_box(new_pos, movement.ivec_dir()) {
                    return false;
                }
            }
            Spot::Wall => return false,
        }

        self.robot_pos = new_pos;
        true
    }

    fn step_all(&mut self, movement: &Vec<Move>) {
        for (i, movement) in movement.iter().enumerate() {
            self.move_robot(*movement);
            println!("\n{i}: {movement:?}");
            // self.print();
        }
    }

    fn all_box_coords(&self) -> Vec<IVec2> {
        let mut coords = Vec::new();
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Spot::Box => {
                        coords.push(IVec2 {
                            x: x as i64,
                            y: y as i64,
                        });
                    }
                    Spot::BoxL => {
                        coords.push(IVec2 {
                            x: x as i64,
                            y: y as i64,
                        });
                    }
                    _ => {}
                }
            }
        }

        coords
    }

    fn print(&self) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if y as i64 == self.robot_pos.y && x as i64 == self.robot_pos.x {
                    print!("@");
                    continue;
                }
                match cell {
                    Spot::Free => print!("."),
                    Spot::Wall => print!("#"),
                    Spot::Box => print!("O"),
                    Spot::BoxL => print!("["),
                    Spot::BoxR => print!("]"),
                }
            }
            println!()
        }
    }
}

#[derive(Debug, Clone)]
struct ParsedInput {
    state: State,
    moves: Vec<Move>,
}

fn parse_input(input: &str, is_part_2: bool) -> ParsedInput {
    let (board_str, moves_str) = input.trim().split_once("\n\n").unwrap();

    let mut board = Vec::new();
    let mut robot_pos = None;
    for (y, line) in board_str.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    row.push(Spot::Free);
                    if is_part_2 {
                        row.push(Spot::Free);
                    }
                }
                '#' => {
                    row.push(Spot::Wall);
                    if is_part_2 {
                        row.push(Spot::Wall);
                    }
                }
                '@' => {
                    robot_pos = Some(IVec2 {
                        x: if is_part_2 { x as i64 * 2 } else { x as i64 },
                        y: y as i64,
                    });

                    row.push(Spot::Free);
                    if is_part_2 {
                        row.push(Spot::Free);
                    }
                }
                'O' => {
                    if is_part_2 {
                        row.push(Spot::BoxL);
                        row.push(Spot::BoxR);
                    } else {
                        row.push(Spot::Box);
                    }
                }
                _ => panic!("unknown dings {c}"),
            }
        }

        board.push(row);
    }

    let mut moves = Vec::new();
    for char in moves_str.chars() {
        if char.is_whitespace() {
            continue;
        }

        let m = match char {
            '<' => Move::Left,
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            _ => unreachable!("bad char {char}"),
        };

        moves.push(m);
    }

    ParsedInput {
        state: State {
            board,
            robot_pos: robot_pos.unwrap(),
        },
        moves,
    }
}
