const INPUT: &str = include_str!("input.txt");

fn main() {
    let input = parse_input();

    println!("part 1");
    part_1(input.clone());
    println!("=======");
    println!("part 2");
    part_2(input);
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Mode {
    Increasing,
    Decreasing,
}

fn part_1(lines: Vec<Vec<i32>>) {
    let safe_lines = lines.into_iter().filter(|v| is_safe(v)).count();
    println!("Safe lines: {safe_lines}")
}

fn part_2(lines: Vec<Vec<i32>>) {
    let safe_lines = lines.into_iter().filter(|v| is_safe_one_err(v)).count();
    println!("Safe lines: {safe_lines}")
}

fn is_safe_one_err(line: &Vec<i32>) -> bool{
    if is_safe(&line) {
        return true;
    }

    let mut removing = 0;
    while removing < line.len() {
        let mut using_line = line.clone();
        using_line.remove(removing);
        removing += 1;

        if is_safe(&using_line){
            return true;
        }
    }

    return false;
}

fn is_safe(line: &Vec<i32>) -> bool {
    let mut last_num = None;
    let mut mode: Option<Mode> = None;

    for num in line {
        let Some(last) = last_num else {
            last_num = Some(num);
            continue;
        };

        if (num - last).abs() > 3 {
            return false;
        }

        let Some(mode) = mode else {
            if num > last {
                mode = Some(Mode::Increasing)
            } else if num < last {
                mode = Some(Mode::Decreasing)
            } else {
                return false;
            }

            last_num = Some(num);
            continue;
        };

        if num == last
            || (num > last && mode == Mode::Decreasing)
            || (num < last && mode == Mode::Increasing)
        {
            return false;
        }

        last_num = Some(num);
    }

    true
}

fn parse_input() -> Vec<Vec<i32>> {
    let mut lines = Vec::new();

    for line in INPUT.lines() {
        let split = line
            .split(' ')
            // .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect::<Vec<i32>>();

        lines.push(split);
    }

    lines
}
