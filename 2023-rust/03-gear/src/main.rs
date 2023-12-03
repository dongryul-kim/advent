fn main() {
    part_two();
}

fn read_input() -> Vec<String> {
    let mut engine = Vec::new();
    for line in std::io::stdin().lines() {
        engine.push(line.unwrap());
    }
    return engine;
}

fn part_one() {
    let engine = read_input();

    let mut sum = 0;
    let re = regex::Regex::new(r"[0-9]+").unwrap();
    for idx in 0..engine.len() {
        for mat in re.find_iter(&engine[idx]) {
            let num: usize = mat.as_str().parse().unwrap();
            let mut flag = false;
            let start = std::cmp::max(mat.start(), 1) - 1;
            let end = std::cmp::min(mat.end() + 1, engine[0].len());
            let is_symbol = |c: char| !(c.is_digit(10) || (c == '.'));

            flag = flag || is_symbol(engine[idx].as_bytes()[start] as char);
            flag = flag || is_symbol(engine[idx].as_bytes()[end - 1] as char);
            if idx != 0 {
                match std::str::from_utf8(&engine[idx - 1].as_bytes()[start..end])
                    .unwrap()
                    .find(is_symbol)
                {
                    Some(_) => flag = true,
                    None => {}
                }
            }
            if idx != engine.len() - 1 {
                match std::str::from_utf8(&engine[idx + 1].as_bytes()[start..end])
                    .unwrap()
                    .find(is_symbol)
                {
                    Some(_) => flag = true,
                    None => {}
                }
            }
            if flag {
                sum += num;
            }
        }
    }
    println!("{sum}");
}

// Precondition: line[col] is a digit
fn find_number(line: &[u8], col: usize) -> usize {
    let first = match std::str::from_utf8(&line[0..col])
        .unwrap()
        .rfind(|c: char| !c.is_digit(10))
        {
            Some(idx) => idx + 1,
            None => 0,
        };
    let last = match std::str::from_utf8(&line[col+1..])
        .unwrap()
        .find(|c: char| !c.is_digit(10))
        {
            Some(idx) => idx+col+1,
            None => line.len(),
        };
    std::str::from_utf8(&line[first..last])
        .unwrap()
        .parse()
        .unwrap()
}

fn search_block_three(line: &[u8], col: usize, nums: &mut Vec<usize>) {
    if col != 0 && (line[col - 1] as char).is_digit(10) {
        nums.push(find_number(line, col-1));
        if !(line[col] as char).is_digit(10) && col != line.len()-1 && (line[col+1] as char).is_digit(10) {
            nums.push(find_number(line, col+1));
        }
    }
    else if (line[col] as char).is_digit(10) {
        nums.push(find_number(line, col));
    }
    else if col != line.len()-1 && (line[col+1] as char).is_digit(10) {
        nums.push(find_number(line, col+1));
    }
}

fn part_two() {
    let engine = read_input();

    let mut sum = 0;
    for row in 0..engine.len() {
        for (col, _) in engine[row].match_indices("*") {
            let mut nums: Vec<usize> = Vec::new();
            let line = &engine[row].as_bytes();

            if col != 0 && (line[col - 1] as char).is_digit(10) {
                nums.push(find_number(line, col-1));
            }
            if col != line.len()-1 && (line[col + 1] as char).is_digit(10) {
                nums.push(find_number(line, col+1));
            }

            if row != 0 {
                search_block_three(&engine[row-1].as_bytes(), col, &mut nums);
            }
            if row != engine.len()-1 {
                search_block_three(&engine[row+1].as_bytes(), col, &mut nums);
            }

            if nums.len() == 2 {
                sum += nums[0] * nums[1];
            }
        }
    }
    println!("{sum}");
}
