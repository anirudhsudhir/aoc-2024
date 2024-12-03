use regex::Regex;

fn main() {
    let input = include_str!("../utils/input_day_3.txt");
    // let input = include_str!("../utils/test_input_day_3_2.txt");
    println!("Day 3 Problem 1 -> {}", problem_1(input));
    println!("Day 3 Problem 2 -> {}", problem_2(input));
}

fn problem_1(inp: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(inp)
        .map(|instr| {
            let (_, [a, b]) = instr.extract();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .map(|(a, b)| a * b)
        .sum()
}

fn problem_2(inp: &str) -> i32 {
    let re = Regex::new(r"don't\(\)((.|\n)*?)do\(\)").unwrap();
    let mod_str = re.replace_all(inp, "");
    problem_1(&mod_str)
}
