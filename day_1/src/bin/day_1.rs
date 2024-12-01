use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Running the binary in the workspace root
const INPUT_PATH: &str = "day_1/utils/input_day_1_1.txt";

fn main() {
    let (mut list_1, mut list_2) = parse_input();
    println!(
        "Day 1 Problem 1: Answer -> {}",
        calc_diff(&mut list_1, &mut list_2)
    );
    println!(
        "Day 1 Problem 2: Answer -> {}",
        similarity_score(&list_1, &list_2)
    );
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let file = File::open(INPUT_PATH).expect("failed to open input file");
    let reader = BufReader::new(file);
    let mut list_1: Vec<i32> = Vec::with_capacity(1000);
    let mut list_2: Vec<i32> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let line = line.expect("failed to read lines from the input file");
        let mut split_line = line.split("   ");

        list_1.push(
            split_line
                .next()
                .expect("failed to iterate over the split line")
                .parse::<i32>()
                .unwrap_or_else(|err| {
                    panic!(
                        "failed to parse first element of line {} to u32 -> {}",
                        line, err
                    )
                }),
        );

        list_2.push(
            split_line
                .next()
                .expect("failed to iterate over the split line")
                .parse::<i32>()
                .unwrap_or_else(|err| {
                    panic!(
                        "failed to parse second element of line {} to u32 -> {}",
                        line, err
                    )
                }),
        );
    }

    (list_1, list_2)
}

fn calc_diff(list_1: &mut [i32], list_2: &mut [i32]) -> i32 {
    list_1.sort();
    list_2.sort();
    let mut total = 0;

    for i in 0..list_1.len() {
        total += (list_2[i] - list_1[i]).abs();
    }

    total
}

fn similarity_score(list_1: &[i32], list_2: &[i32]) -> i32 {
    let mut similarity_sum = 0;
    let mut freq: HashMap<i32, i32> = HashMap::with_capacity(100);

    for num in list_2 {
        if let Some(count) = freq.get(num) {
            freq.insert(num.to_owned(), count + 1);
        } else {
            freq.insert(num.to_owned(), 1);
        }
    }

    for num in list_1 {
        if let Some(count) = freq.get(num) {
            similarity_sum += num * count;
        }
    }

    similarity_sum
}
