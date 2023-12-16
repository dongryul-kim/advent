fn main() {
    part_two();
}

fn read() -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::new();
    for (i, line) in std::io::stdin().lines().enumerate() {
        for (j, c) in line.unwrap().chars().enumerate() {
            if c == '#' {
                res.push((i, j));
            }
        }
    }
    return res;
}

fn calc_dist(mut x: Vec<usize>, factor: usize) -> usize {
    x.sort();
    let mut sum = 0;
    for i in 0..(x.len() - 1) {
        let dist = match x[i + 1] - x[i] {
            0 => 0,
            _ => factor * (x[i + 1] - x[i] - 1) + 1,
        };
        sum += (i + 1) * (x.len() - i - 1) * dist;
    }
    return sum;
}

fn part_one() {
    let data = read();
    let x_coords = data.iter().map(|&(x, _)| x).collect();
    let y_coords = data.iter().map(|&(_, y)| y).collect();
    println!("{}", calc_dist(x_coords, 2) + calc_dist(y_coords, 2));
}

fn part_two() {
    let data = read();
    let x_coords = data.iter().map(|&(x, _)| x).collect();
    let y_coords = data.iter().map(|&(_, y)| y).collect();
    println!(
        "{}",
        calc_dist(x_coords, 1_000_000) + calc_dist(y_coords, 1_000_000)
    );
}
