const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1");
    part_1();

    println!("=======");
    println!("part 2");
    part_2();
}

fn part_1() {
    let input = parse_input();
    let word = &['X', 'M', 'A', 'S'];
    find_word(input, word);
}

fn part_2() {
    let input = parse_input();

    let mask = vec![
        vec![Some('M'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('S')],
    ];

    let mask_flipped_h = vec![
        vec![Some('S'), None, Some('M')],
        vec![None, Some('A'), None],
        vec![Some('S'), None, Some('M')],
    ];

    let mask_flipped_hv = vec![
        vec![Some('S'), None, Some('S')],
        vec![None, Some('A'), None],
        vec![Some('M'), None, Some('M')],
    ];

    let mask_flipped_hv_2 = vec![
        vec![Some('M'), None, Some('M')],
        vec![None, Some('A'), None],
        vec![Some('S'), None, Some('S')],
    ];

    let masks = &[
        mask, mask_flipped_h, mask_flipped_hv, mask_flipped_hv_2
    ];

    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for mask in masks{
                if is_mask_match(&input, &mask, x, y){
                    count+= 1;
                    // println!("Found mask x{x} y{y}");
                }
            }
        }
    }

    println!("Result: {count}")
}

// Takes in a "mul(1,2)" str and returns the result
fn parse_input() -> Vec<Vec<char>> {
    // just the numbers (e.g "1,2")
    let mut output = Vec::new();
    for line in INPUT.lines() {
        output.push(line.chars().collect());
    }

    output
}

const DIRECTIONS: &[(i32, i32)] = &[
    (1, 0),  // horizontal
    (0, 1),  // vertical
    (1, 1),  // diagonal right
    (-1, 1), // diagonal left
];

fn find_word(input: Vec<Vec<char>>, word: &[char]) {
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            for (move_x, move_y) in DIRECTIONS {
                if is_word_match(&input, word, x, y, *move_x, *move_y) {
                    // println!("Found word {x}.{y}: dir: {move_x}.{move_y}");
                    count += 1;
                }

                // Reverse
                if is_word_match(&input, word, x, y, -*move_x, -*move_y) {
                    // println!("Found word reverse {x}.{y}: dir: {move_x}.{move_y}");
                    count += 1;
                }
            }
        }
    }

    println!("Result: {count}")
}

fn is_word_match(
    input: &Vec<Vec<char>>,
    word: &[char],
    start_x: usize,
    start_y: usize,
    move_x: i32,
    move_y: i32,
) -> bool {
    let mut cur_x = start_x as i32;
    let mut cur_y = start_y as i32;

    for c in word {
        if cur_x < 0 || cur_x >= input[start_y].len() as i32 {
            return false;
        }

        if cur_y < 0 || cur_y >= input.len() as i32 {
            return false;
        }

        if input[cur_y as usize][cur_x as usize] != *c {
            return false;
        }

        cur_x += move_x;
        cur_y += move_y;
    }

    true
}

fn is_mask_match(
    input: &Vec<Vec<char>>,
    char_mask: &Vec<Vec<Option<char>>>,
    start_x: usize,
    start_y: usize,
) -> bool {
    for mask_y in 0..char_mask.len() {
        for mask_x in 0..char_mask[mask_y].len() {
            let x = mask_x + start_x;
            let y = mask_y + start_y;

            if y >= input.len() || x >= input[y].len() {
                return false;
            }

            if let Some(filter_char) = char_mask[mask_y][mask_x] {
                if filter_char != input[y][x] {
                    return false;
                }
            }
        }
    }

    true
}
