fn main() {
    let inp = include_str!("../utils/input_day_7.txt");
    let map = parse_inp(inp);

    let part_1 = map
        .iter()
        .filter(|(k, v)| recursive_validation_part_1(0, v, 0, v.len(), k))
        .map(|(k, _)| k)
        .sum::<u64>();

    let part_2 = map
        .iter()
        .filter(|(k, v)| recursive_validation_part_2(0, v, 0, v.len(), k))
        .map(|(k, _)| k)
        .sum::<u64>();

    println!("Day 7 Problem 1 -> {part_1}");
    println!("Day 7 Problem 2 -> {part_2}");
}

fn parse_inp(inp: &str) -> Vec<(u64, Vec<u64>)> {
    let mut map = Vec::with_capacity(100);
    inp.lines()
        .map(|line| {
            let mut line_split = line.split(":");
            (line_split.next().unwrap(), line_split.next().unwrap())
        })
        .map(|(val, nums)| {
            map.push((
                val.parse::<u64>().unwrap(),
                nums.split(" ")
                    .filter(|num| !num.is_empty())
                    .map(|num| num.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            ))
        })
        .for_each(drop);

    map
}

fn recursive_validation_part_1(
    calc_val: u64,
    nums: &Vec<u64>,
    start_index: usize,
    nums_len: usize,
    check: &u64,
) -> bool {
    if start_index >= nums_len {
        return calc_val == *check;
    }
    recursive_validation_part_1(
        calc_val + nums[start_index],
        nums,
        start_index + 1,
        nums_len,
        check,
    ) || recursive_validation_part_1(
        (if calc_val == 0 { 1 } else { calc_val }) * nums[start_index],
        nums,
        start_index + 1,
        nums_len,
        check,
    )
}

fn recursive_validation_part_2(
    calc_val: u64,
    nums: &Vec<u64>,
    start_index: usize,
    nums_len: usize,
    check: &u64,
) -> bool {
    if start_index >= nums_len {
        return calc_val == *check;
    }
    recursive_validation_part_2(
        calc_val + nums[start_index],
        nums,
        start_index + 1,
        nums_len,
        check,
    ) || recursive_validation_part_2(
        (if calc_val == 0 { 1 } else { calc_val }) * nums[start_index],
        nums,
        start_index + 1,
        nums_len,
        check,
    ) || recursive_validation_part_2(
        if calc_val == 0 {
            nums[start_index]
        } else {
            (calc_val.to_string() + &nums[start_index].to_string())
                .parse()
                .unwrap()
        },
        nums,
        start_index + 1,
        nums_len,
        check,
    )
}
