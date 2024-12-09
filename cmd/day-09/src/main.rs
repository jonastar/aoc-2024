use std::fmt::{Display, Write};

const INPUT: &str = include_str!("input.txt");
const EXAMPLE_INPUT: &str = r#"
12983712899287
"#;
// 00...111...2...333.44.5555.6666.777.888899

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
    let mut cloned_input = parsed_input.clone();

    // Compact
    while compact_step_part1(&mut cloned_input) {}

    // checksum
    let result = checksum(&cloned_input);

    println!("Result: {result}");
}

fn compact_step_part1(input: &mut ParsedInput) -> bool {
    let last_block_id = input.blocks.last().unwrap().id;

    let first_free_space = input
        .blocks
        .iter()
        .position(|v| v.free_space_right > 0)
        .unwrap();
    if first_free_space == input.blocks.len() - 1 {
        // we reached the end
        return false;
    }

    if input.blocks[first_free_space].id == last_block_id {
        // grow left block segment
        input.blocks[first_free_space].length += 1;
        input.blocks[first_free_space].free_space_right -= 1;
    } else {
        // Insert new block segment
        let new_block_free_space = input.blocks[first_free_space].free_space_right - 1;

        input.blocks.insert(
            first_free_space + 1,
            Block {
                free_space_right: new_block_free_space,
                id: last_block_id,
                length: 1,
            },
        );

        input.blocks[first_free_space].free_space_right = 0;
    };

    // shrink right block
    let last_block = input.blocks.last_mut().unwrap();
    last_block.length -= 1;
    last_block.free_space_right += 1;
    if last_block.length == 0 {
        // Remove empty block and carry over free space
        let free_space = last_block.free_space_right;
        input.blocks.pop();
        let new_last = input.blocks.last_mut().unwrap();
        new_last.free_space_right += free_space;
    }

    return true;
}

fn part_2(parsed_input: &ParsedInput) {
    let mut cloned_input = parsed_input.clone();

    // Compact
    compact_part2(&mut cloned_input);

    // checksum
    let result = checksum(&cloned_input);

    println!("Result: {result}");
}

fn compact_part2(input: &mut ParsedInput) {
    let last_block = input.blocks.len();

    // println!("Original: {input}");
    for id in (0..last_block).rev() {
        let index = input
            .blocks
            .iter()
            .position(|v| v.id as usize == id)
            .unwrap();

        let Some(free_index) = input
            .blocks
            .iter()
            .position(|v| v.free_space_right >= input.blocks[index].length)
        else {
            continue;
        };

        if free_index >= index {

            continue;
        }

        // println!("Moving {index} ({id}) to index {free_index}");

        let mut block = input.blocks.remove(index);
        let old_right_free_space = block.free_space_right;
        input.blocks[index - 1].free_space_right += old_right_free_space + block.length;

        let old_left_free_space = input.blocks[free_index].free_space_right;
        input.blocks[free_index].free_space_right = 0;


        block.free_space_right = old_left_free_space - block.length;
        input.blocks.insert(free_index + 1, block);
        // println!("Step:     {input}");

        // we do not need to carry over free space on the right as we dont use that for anything
    }
}

fn checksum(input: &ParsedInput) -> u128 {
    let mut result = 0;
    let mut position = 0;
    for block in &input.blocks {
        for _ in 0..block.length {
            let inner = position as u128 * block.id as u128;
            // println!("{position} * {} = {inner}", block.id);
            result += inner;

            position += 1;
        }
        position += block.free_space_right;
    }

    result
}

#[derive(Clone)]
struct Block {
    free_space_right: u32,
    id: u32,
    length: u32,
}

#[derive(Clone)]
struct ParsedInput {
    blocks: Vec<Block>,
}

fn parse_input(input: &str) -> ParsedInput {
    println!("{}", input.trim().len());
    let mut iter = input.trim().chars();

    let mut blocks = Vec::new();
    let mut id = 0u32;
    loop {
        let Some(length_char) = iter.next() else {
            break;
        };

        let length = length_char.to_digit(10).unwrap();
        assert!(length > 0);

        let free_space = iter.next().unwrap_or('0').to_digit(10).unwrap();
        blocks.push(Block {
            id,
            length,
            free_space_right: free_space,
        });
        id += 1;
        // println!("{id}")
    }

    ParsedInput { blocks }
}

impl Display for ParsedInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for block in &self.blocks {
            for _ in 0..block.length {
                let mut s = block.id.to_string();
                if s.len() < 2{
                    s = format!("0{s}");
                }
                f.write_str(&s)?;
                // f.write_char(char::from_digit(block.id, 10).unwrap())?;
            }
            for _ in 0..block.free_space_right {
                f.write_char('.')?;
                f.write_char('.')?;
            }
        }

        Ok(())
    }
}
