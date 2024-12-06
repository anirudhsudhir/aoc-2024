use std::collections::HashSet;

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let inp = include_str!("../utils/input_day_6.txt");
    println!("Day 6 Part 1 -> {}", problem_1(inp));
}

fn problem_1(inp: &str) -> usize {
    let mat: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();
    let mut initial_guard_pos = (0, 0);
    for (index, row) in mat.iter().enumerate() {
        if let Some(y) = row.iter().position(|&ch| ch == '^') {
            initial_guard_pos = (index, y);
            break;
        }
    }

    distinct_locs(mat, initial_guard_pos, Dir::Up, HashSet::new())
}

fn distinct_locs(
    mat: Vec<Vec<char>>,
    (x, y): (usize, usize),
    mut dir: Dir,
    mut locs: HashSet<(usize, usize)>,
) -> usize {
    let (mut cur_x, mut cur_y) = (x, y);
    let (mut upd_x, mut upd_y);

    loop {
        match dir {
            Dir::Up => {
                if cur_x == 0 {
                    return locs.len();
                }
                (upd_x, upd_y) = (cur_x - 1, cur_y);
                if (0..mat.len()).contains(&upd_x) && (0..mat[0].len()).contains(&upd_y) {
                    if mat[upd_x][upd_y] == '#' {
                        dir = Dir::Right;
                        continue;
                    } else {
                        (cur_x, cur_y) = (upd_x, upd_y);
                        locs.insert((cur_x, cur_y));
                    }
                } else {
                    return locs.len();
                }
            }
            Dir::Right => {
                (upd_x, upd_y) = (cur_x, cur_y + 1);
                if (0..mat.len()).contains(&upd_x) && (0..mat[0].len()).contains(&upd_y) {
                    if mat[upd_x][upd_y] == '#' {
                        dir = Dir::Down;
                        continue;
                    } else {
                        (cur_x, cur_y) = (upd_x, upd_y);
                        locs.insert((cur_x, cur_y));
                    }
                } else {
                    return locs.len();
                }
            }
            Dir::Down => {
                (upd_x, upd_y) = (cur_x + 1, cur_y);
                if (0..mat.len()).contains(&upd_x) && (0..mat[0].len()).contains(&upd_y) {
                    if mat[upd_x][upd_y] == '#' {
                        dir = Dir::Left;
                        continue;
                    } else {
                        (cur_x, cur_y) = (upd_x, upd_y);
                        locs.insert((cur_x, cur_y));
                    }
                } else {
                    return locs.len();
                }
            }
            Dir::Left => {
                if cur_y == 0 {
                    return locs.len();
                }
                (upd_x, upd_y) = (cur_x, cur_y - 1);
                if (0..mat.len()).contains(&upd_x) && (0..mat[0].len()).contains(&upd_y) {
                    if mat[upd_x][upd_y] == '#' {
                        dir = Dir::Up;
                        continue;
                    } else {
                        (cur_x, cur_y) = (upd_x, upd_y);
                        locs.insert((cur_x, cur_y));
                    }
                } else {
                    return locs.len();
                }
            }
        };
    }
}
