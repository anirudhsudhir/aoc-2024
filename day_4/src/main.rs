fn main() {
    let inp = include_str!("../utils/input_day_4.txt");
    println!("Day 4 Problem 1 -> {}", problem_1(inp));
    println!("Day 4 Problem 2 -> {}", problem_2(inp));
}

fn problem_1(inp: &str) -> usize {
    let mat: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();
    let height = mat.len();
    let width = mat[0].len();

    let hor_count = inp
        .lines()
        .map(|line| line.matches("XMAS").count() + line.matches("SAMX").count())
        .sum::<usize>();

    let mut ver_count = 0;
    for col in 0..width {
        let mut i = 0;
        let mut substr = String::new();
        loop {
            if i >= height {
                break;
            }
            substr.push(mat[i][col]);
            i += 1;
        }
        ver_count += substr.matches("XMAS").count() + substr.matches("SAMX").count();
    }

    // row 0 right diagonals
    let mut diagonal_count = 0;
    for col in 0..width - 1 {
        let substr = gen_r_diag_substr(0, col, width, height, &mat);
        diagonal_count += substr.matches("XMAS").count() + substr.matches("SAMX").count();
    }

    // col 0 right diagonals
    // starting from 1 as primary diagonal already traversed
    for row in 1..height - 1 {
        let substr = gen_r_diag_substr(row, 0, width, height, &mat);
        diagonal_count += substr.matches("XMAS").count() + substr.matches("SAMX").count();
    }

    // row 0 left diagonals
    for col in 1..=width {
        let substr = gen_l_diag_substr(0, width - col, height, &mat);
        diagonal_count += substr.matches("XMAS").count() + substr.matches("SAMX").count();
    }

    // col (width-1) left diagonals
    for row in 1..height - 1 {
        let substr = gen_l_diag_substr(row, width - 1, height, &mat);
        diagonal_count += substr.matches("XMAS").count() + substr.matches("SAMX").count();
    }

    // dbg!(hor_count, ver_count, diagonal_count);
    hor_count + ver_count + diagonal_count
}

fn problem_2(inp: &str) -> i32 {
    let mat: Vec<Vec<char>> = inp.lines().map(|line| line.chars().collect()).collect();
    let height = mat.len();
    let width = mat[0].len();
    let mut count = 0;

    let (mut i, mut j) = (1, 1);
    loop {
        if i >= height {
            break;
        }
        loop {
            if j >= width {
                break;
            }
            if mat[i][j] == 'A' {
                let (ul_i, ul_j) = (i - 1, j - 1);
                let (ur_i, ur_j) = (i - 1, j + 1);
                let (dl_i, dl_j) = (i + 1, j - 1);
                let (dr_i, dr_j) = (i + 1, j + 1);

                if (0..height).contains(&ul_i)
                    && (0..height).contains(&ur_i)
                    && (0..height).contains(&dl_i)
                    && (0..height).contains(&dr_i)
                    && (0..width).contains(&ul_j)
                    && (0..width).contains(&ur_j)
                    && (0..width).contains(&dl_j)
                    && (0..width).contains(&dr_j)
                {
                    let mut l_diag = String::new();
                    let mut r_diag = String::new();
                    l_diag.push(mat[ul_i][ul_j]);
                    l_diag.push(mat[i][j]);
                    l_diag.push(mat[dr_i][dr_j]);

                    r_diag.push(mat[ur_i][ur_j]);
                    r_diag.push(mat[i][j]);
                    r_diag.push(mat[dl_i][dl_j]);

                    if (l_diag.contains("MAS") || l_diag.contains("SAM"))
                        && (r_diag.contains("MAS") || r_diag.contains("SAM"))
                    {
                        count += 1;
                    }
                }
            }
            j += 1;
        }
        j = 1;
        i += 1;
    }

    count
}

fn gen_r_diag_substr(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    mat: &[Vec<char>],
) -> String {
    let (mut i, mut j) = (row, col);
    let mut subst = String::new();
    loop {
        if i >= height || j >= width {
            break;
        }
        subst.push(mat[i][j]);
        i += 1;
        j += 1;
    }
    subst
}

fn gen_l_diag_substr(row: usize, col: usize, height: usize, mat: &[Vec<char>]) -> String {
    let (mut i, mut j) = (row, col);
    let mut subst = String::new();
    loop {
        if i >= height {
            break;
        }
        subst.push(mat[i][j]);
        i += 1;
        if j == 0 {
            break;
        }
        j -= 1;
    }
    subst
}
