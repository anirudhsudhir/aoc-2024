use std::collections::HashMap;
use std::fs;
use std::iter::zip;

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
    let reader = fs::read_to_string(INPUT_PATH)
        .unwrap_or_else(|err| panic!("failed to read input file to string -> {err}"));
    let mut list_1: Vec<i32> = Vec::with_capacity(1000);
    let mut list_2: Vec<i32> = Vec::with_capacity(1000);

    for line in reader.lines() {
        let mut split_line = line.split("   ");

        list_1.push(
            split_line
                .next()
                .expect("failed to iterate over the split line")
                .parse()
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
                .parse()
                .unwrap_or_else(|err| {
                    panic!(
                        "failed to parse second element of line {} to u32 -> {}",
                        line, err
                    )
                }),
        );
    }
    //
    // reader
    //     .lines()
    //     .map(|line| {
    //         let mut split_line = line.split("   ");
    //         list_1.push(
    //             split_line
    //                 .next()
    //                 .expect("failed to iterate over the split line")
    //                 .parse()
    //                 .unwrap_or_else(|err| {
    //                     panic!(
    //                         "failed to parse first element of line {} to u32 -> {}",
    //                         line, err
    //                     )
    //                 }),
    //         );
    //
    //         list_2.push(
    //             split_line
    //                 .next()
    //                 .expect("failed to iterate over the split line")
    //                 .parse()
    //                 .unwrap_or_else(|err| {
    //                     panic!(
    //                         "failed to parse second element of line {} to u32 -> {}",
    //                         line, err
    //                     )
    //                 }),
    //         );
    //     })
    //     .for_each(drop);

    (list_1, list_2)
}

fn calc_diff(list_1: &mut [i32], list_2: &mut [i32]) -> i32 {
    list_1.sort();
    list_2.sort();

    zip(list_2, list_1)
        .map(|(a, b)| (a.to_owned() - b.to_owned()).abs())
        .sum()
}

fn similarity_score(list_1: &[i32], list_2: &[i32]) -> i32 {
    let mut freq: HashMap<i32, i32> = HashMap::with_capacity(500);

    list_2
        .iter()
        .map(|num| {
            if let Some(count) = freq.get(num) {
                freq.insert(num.to_owned(), count + 1);
            } else {
                freq.insert(num.to_owned(), 1);
            }
        })
        .for_each(drop);

    list_1.iter().map(|a| freq.get(a).unwrap_or(&0) * a).sum()
}
