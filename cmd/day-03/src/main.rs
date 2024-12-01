const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1");
    part_1();

    println!("=======");
    println!("part 2");
    part_2();
}

fn part_1() {
    let mut remainder = &INPUT[..];
    let mut sum = 0;
    loop {
        let Some(next_open_index) = remainder.find("mul(") else {
            break;
        };

        let buf_start_open = &remainder[next_open_index..];
        remainder = &remainder[next_open_index + "mul(".len()..];

        let Some(next_close_index) = buf_start_open.find(")") else {
            break;
        };

        if let Some(mul_result) = parse_mul_instruction(&buf_start_open[..=next_close_index]){
            sum += mul_result;
        }
    }

    println!("Result: {sum}")
}

fn part_2() {
    let mut remainder = &INPUT[..];
    let mut sum = 0;
    loop {
        let next_dont = remainder.find("don't()");
        let Some(next_open_index) = remainder.find("mul(") else {
            break;
        };

        let buf_start_open = &remainder[next_open_index..];

        if let Some(next_dont) = next_dont {
            if next_dont < next_open_index {
                // find when to re-enable
                remainder = &remainder[next_open_index..];
                let Some(next_do_index) = remainder.find("do()") else {
                    break;
                };

                remainder = &remainder[next_do_index..];
                continue;
            }
        }

        remainder = &remainder[next_open_index + "mul(".len()..];

        

        let Some(next_close_index) = buf_start_open.find(")") else {
            break;
        };

        if let Some(mul_result) = parse_mul_instruction(&buf_start_open[..=next_close_index]){
            sum += mul_result;
        }
    }

    println!("Result: {sum}")
}

// Takes in a "mul(1,2)" str and returns the result
fn parse_mul_instruction(s: &str) -> Option<i32> {
    // just the numbers (e.g "1,2")
    let arg_buffer_str = &s["mul(".len()..s.len()-1];
    
    let comma_index = arg_buffer_str.find(",")?;
    let num_pre_comma = &arg_buffer_str[..comma_index].parse::<i32>().ok()?;
    let num_post_comma = &arg_buffer_str[comma_index + 1..].parse::<i32>().ok()?;

    Some(num_pre_comma * num_post_comma)
}
