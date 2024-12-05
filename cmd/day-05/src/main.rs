const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part 1");
    part_1();

    println!("=======");
    println!("part 2");
    part_2();
}

fn part_1() {
    let mut total = 0;

    let parsed = parse_input();
    for update in &parsed.updates {
        if parsed.is_valid_update(&update) {
            let middle_index = update.len() / 2;
            total += update[middle_index];
        }
    }

    println!("Total: {total}");
}

fn part_2() {
    let mut total = 0;

    let parsed = parse_input();
    for update in &parsed.updates {
        if !parsed.is_valid_update(&update) {
            let fixed = parsed.fix_update(&update);
            let middle_index = fixed.len() / 2;
            total += fixed[middle_index];
        }
    }

    println!("Total: {total}");
}

struct OrderRule {
    before: i32,
    after: i32,
}

struct ParsedInput {
    rules: Vec<OrderRule>,
    updates: Vec<Vec<i32>>,
}

fn parse_input() -> ParsedInput {
    let mut parsing_updates = false;

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    for line in INPUT.lines() {
        if line.is_empty() {
            parsing_updates = true;
            continue;
        }

        if parsing_updates {
            let update = line.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
            updates.push(update);
        } else {
            let mut split = line.split('|');
            let x = split.next().unwrap();
            let y = split.next().unwrap();

            rules.push(OrderRule {
                before: x.parse().unwrap(),
                after: y.parse().unwrap(),
            });
        }
    }

    ParsedInput { rules, updates }
}

impl ParsedInput {
    fn check_order_rules(&self, wants_before: i32, wants_after: i32) -> bool {
        for rule in &self.rules {
            if rule.before == wants_before && rule.after == wants_after {
                return true;
            } else if rule.after == wants_before && rule.before == wants_after {
                return false;
            }
        }

        true
    }

    fn find_bad_update_index(&self, update: &Vec<i32>) -> Option<(usize, usize)> {
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                // make sure j prints after i
                if !self.check_order_rules(update[i], update[j]) {
                    return Some((i,j));
                }
            }
        }

        None
    }

    fn is_valid_update(&self, update: &Vec<i32>) -> bool {
        self.find_bad_update_index(update).is_none()
    }

    fn fix_update(&self, update: &Vec<i32>) -> Vec<i32> {
        let mut fixed = update.clone();

        // Swap bad order until it's correct... yeah probably a faster way to do this
        while let Some((wants_before_i, wants_after_i)) = self.find_bad_update_index(&fixed){
            // Simply swap them i guess
            let temp  = fixed[wants_before_i];
            fixed[wants_before_i] = fixed[wants_after_i];
            fixed[wants_after_i] = temp;
        }

        fixed
    }
}
