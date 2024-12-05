use std::collections::HashMap;

fn main() {
    let inp = include_str!("../utils/input_day_5.txt");
    let (ans_1, ans_2) = day_5(inp);
    println!("Day 5 Problem 1 -> {}", ans_1);
    println!("Day 5 Problem 2 -> {}", ans_2);
}

fn day_5(inp: &str) -> (i32, i32) {
    let mut inp_split = inp.split("\n\n");

    let mut rule_store: HashMap<i32, Vec<i32>> = HashMap::new();
    // rules
    inp_split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split("|"))
        .map(|mut nums| {
            let first = nums.next().unwrap().parse::<i32>().unwrap();
            let second = nums.next().unwrap().parse::<i32>().unwrap();
            if let Some(rules) = rule_store.get_mut(&first) {
                rules.push(second);
            } else {
                rule_store.insert(first, vec![second]);
            }
        })
        .for_each(drop);

    let mut problem_1_answer = 0;
    let mut problem_2_answer = 0;

    let test = inp_split.next().unwrap().lines();
    let mut test_vec = Vec::new();
    for line in test.clone() {
        let line_split = line.split(",");
        let mut row_vec: Vec<i32> = Vec::with_capacity(5);
        for ele in line_split {
            let num_ele = ele.parse().unwrap();
            row_vec.push(num_ele);
        }
        test_vec.push(row_vec);
    }

    // problem_1
    for row in test_vec.clone() {
        let mut valid = true;
        let mut index = 0;
        for ele in 0..row.len() {
            let num_ele = row[ele];
            valid = row[0..index].iter().all(|num| {
                if let Some(rules) = rule_store.get(&num_ele) {
                    return !rules.contains(num);
                }
                true
            });
            if !valid {
                break;
            }
            index += 1;
        }
        if valid {
            problem_1_answer += row[index / 2];
        }
    }

    // problem_2
    for mut row in test_vec {
        let mut perform_sort = false;
        for j in 0..row.len() {
            for k in 0..row.len() - j {
                if let Some(rules) = rule_store.get(&row[k]) {
                    if k > 0 && rules.contains(&row[k - 1]) {
                        perform_sort = true;
                        row.swap(k, k - 1);
                    }
                }
            }
        }
        if perform_sort {
            problem_2_answer += row[row.len() / 2];
        }
    }

    (problem_1_answer, problem_2_answer)
}
