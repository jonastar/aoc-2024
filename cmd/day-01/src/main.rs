const INPUT: &str = include_str!("input.txt");

fn main() {
    let (left, right) = parse_lists();

    assert!(left.len() == right.len());

    println!("part 1");
    part_1(left.clone(), right.clone());
    println!("=======");
    println!("part 2");
    part_2(left, right);
}

fn part_1(left: Vec<i32>, right: Vec<i32>) {
    let mut total_distance = 0;
    for i in 0..left.len() {
        let distance = (left[i] - right[i]).abs();
        total_distance += distance;
    }

    println!("Total: {total_distance}");
}

fn part_2(left: Vec<i32>, right: Vec<i32>) {
    let mut total_similarity = 0;
    for i in 0..left.len() {
        let num = left[i];
        let count = right.iter().filter(|v| **v == num).count() as i32;
        let similarity = num * count;
        total_similarity += similarity;
    }

    println!("Total: {total_similarity}");
}

fn parse_lists() -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in INPUT.lines() {
        let split = line
            .split(' ')
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>();
        assert!(split.len() == 2);

        left.push(split[0].parse().unwrap());
        right.push(split[1].parse().unwrap());
    }

    left.sort();
    right.sort();

    (left, right)
}
