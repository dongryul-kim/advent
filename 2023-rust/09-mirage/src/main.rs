use itertools::Itertools;

fn main() {
    part_two();
}

fn read() -> Vec<Vec<i64>> {
    std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

fn extrapolate(x: &Vec<i64>) -> i64 {
    if x.iter().all(|&v| v == 0) {
        return 0;
    }
    let new_x: Vec<i64> = x.iter().tuple_windows().map(|(&a, &b)| b - a).collect();
    return extrapolate(&new_x) + x[x.len() - 1];
}

fn extrapolate_back(x: &Vec<i64>) -> i64 {
    if x.iter().all(|&v| v == 0) {
        return 0;
    }
    let new_x: Vec<i64> = x.iter().tuple_windows().map(|(&a, &b)| b - a).collect();
    return x[0] - extrapolate_back(&new_x);
}

fn part_one() {
    let data = read();
    let mut sum = 0;
    for line in data {
        sum += extrapolate(&line);
    }
    println!("{sum}");
}

fn part_two() {
    let data = read();
    let mut sum = 0;
    for line in data {
        sum += extrapolate_back(&line);
    }
    println!("{sum}");
}
