fn get_adjancent(m: Vec<Vec<char>>, i: usize, j: usize, num_size: usize) -> Vec<char> {
    let mut adj: Vec<char> = Vec::new();
    for k in j..j + num_size {
        adj.insert(0, m[i][k]);
        if i + 1 < m.len() {
            adj.insert(0, m[i + 1][k]);
        }

        if i > 0 {
            adj.insert(0, m[i - 1][k]);
        }

        if k + 1 < m[0].len() {
            adj.insert(0, m[i][k + 1]);
        }

        if k > 0 {
            adj.insert(0, m[i][k - 1]);
        }

        if i + 1 < m.len() && k + 1 < m[0].len() {
            adj.insert(0, m[i + 1][k + 1]);
        }

        if i + 1 < m.len() && k > 0 {
            adj.insert(0, m[i + 1][k - 1]);
        }

        if i > 0 && k + 1 < m[0].len() {
            adj.insert(0, m[i - 1][k + 1]);
        }

        if i > 0 && k > 0 {
            adj.insert(0, m[i - 1][k - 1]);
        }
    }

    return adj
        .iter()
        .filter(|&&c| c != '.' && !c.is_digit(10))
        .cloned()
        .collect();
}

fn get_num(m: Vec<Vec<char>>, i: usize, j: usize) -> String {
    let mut num: String = String::new();
    let col = m[i].len();
    for k in j..col {
        if m[i][k].is_digit(10) {
            num.push_str(&m[i][k].to_string())
        } else {
            break;
        }
    }

    return num;
}

pub fn solve_part_one(lines: Vec<String>) {
    let m: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut sum = 0;
    for (i, row) in m.clone().iter().enumerate() {
        let mut skip: usize = 0;
        for (j, _col) in row.iter().enumerate() {
            if skip > 0 {
                skip -= 1;
            } else {
                if m[i][j].is_digit(10) {
                    let num: String = get_num(m.clone(), i, j);
                    skip += num.len();
                    let adj: Vec<char> = get_adjancent(m.clone(), i, j, num.len());

                    if adj.len() > 0 {
                        sum += num.parse::<i64>().unwrap();
                    }
                }
            }
        }
    }
    println!("day3_1 {}", sum)
}

fn get_num_pt2(m: Vec<Vec<char>>, i: usize, j: usize) -> i64 {
    let mut ret = String::new();

    let mut k1 = j;
    while m[i][k1].is_digit(10) {
        ret.push(m[i][k1]);
        if k1 == 0 {
            break;
        }
        k1 -= 1;
    }

    ret = ret.chars().rev().collect();

    let mut k2 = j + 1;
    while m[i][k2].is_digit(10) {
        ret.push(m[i][k2]);
        if k2 + 1 == m[i].len() {
            break;
        }
        k2 += 1;
    }

    return ret.parse::<i64>().unwrap();
}

fn is_in_bounds(m: i32, n: i32, i: i32, j: i32) -> bool {
    return i >= 0 && j >= 0 && i < m && j < n;
}

fn scan_nums(m: Vec<Vec<char>>, i: usize, j: usize) -> Vec<i64> {
    let mut nums: Vec<i64> = Vec::new();

    let indexes: [[i32; 2]; 8] = [
        [1, 0],
        [0, 1],
        [-1, 0],
        [0, -1],
        [1, 1],
        [-1, -1],
        [1, -1],
        [-1, 1],
    ];

    for ele in indexes {
        let ii = i as i32 + ele[0];
        let jj = j as i32 + ele[1];
        if is_in_bounds(m.len() as i32, m[i].len() as i32, ii, jj) {
            if m[ii as usize][jj as usize].is_digit(10) {
                let num = get_num_pt2(m.clone(), ii as usize, jj as usize);
                nums.insert(0, num);
            }
        }
    }

    nums.sort();
    nums.dedup();
    return nums;
}

pub fn solve_part_two(lines: Vec<String>) {
    let m: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut sum = 0;
    for (i, row) in m.clone().iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if m[i][j] == '*' {
                let nums = scan_nums(m.clone(), i, j);
                if nums.len() == 2 {
                    sum += nums[0] * nums[1];
                }
            }
        }
    }
    println!("day3_1 {}", sum)
}
