fn main() {
    let input = include_str!("../../day_2/utils/input_day_2.txt");
    println!("Day 2 Problem 1 -> {}", problem_1(input));
    println!("Day 2 Problem 2 -> {}", problem_2(input));
}

fn problem_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            if valid_level(&line.split(" ").collect()) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn problem_2(input: &str) -> i32 {
    input
        .lines()
        .filter(|line| valid_level_with_tolerance(line))
        .count() as i32
}

fn valid_level(level: &Vec<&str>) -> bool {
    let mut incr = None;
    for i in 0..level.len() - 1 {
        let diff = level[i].parse::<i32>().expect("failed to parse str to i32")
            - level[i + 1]
                .parse::<i32>()
                .expect("failed to parse str to i32");
        if !(1..4).contains(&diff.abs()) {
            return false;
        }
        if let Some(level_state) = incr {
            if level_state != (diff > 0) {
                return false;
            }
        } else {
            incr = Some(diff > 0);
        }
    }
    true
}

fn valid_level_with_tolerance(level: &str) -> bool {
    let level_split: Vec<&str> = level.split(" ").collect();
    if valid_level(&level_split) {
        return true;
    }

    for i in 0..level_split.len() {
        let mut curr_removed: Vec<&str> = level_split.clone();
        curr_removed.remove(i);

        if valid_level(&curr_removed) {
            return true;
        }
    }

    false
}
