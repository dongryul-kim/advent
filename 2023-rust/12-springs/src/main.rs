fn main() {
    part_two();
}

fn read() -> Vec<(String, Vec<usize>)> {
    let mut data: Vec<(String, Vec<usize>)> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut iter = line.split_ascii_whitespace();
        let s = iter.next().unwrap();
        let nums = iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        data.push((s.to_string(), nums));
    }
    return data;
}

fn count(s: &[u8], x: &[usize]) -> usize {
    let mut res = vec![vec![0; x.len() + 1]; s.len() + 1];
    for idx_s in (0..s.len() + 1).rev() {
        let s = &s[idx_s..];
        for idx_x in (0..x.len() + 1).rev() {
            let x = &x[idx_x..];
            if s.len() == 0 {
                res[idx_s][idx_x] = match x.len() == 0 {
                    true => 1,
                    false => 0,
                };
                continue;
            }
            let mut sum = 0;
            if s[0] != '#' as u8 {
                sum += res[idx_s + 1][idx_x];
            }
            if s[0] != '.' as u8 {
                if x.len() != 0 && s.len() >= x[0] && s[..x[0]].iter().all(|&c| c != '.' as u8) {
                    if s.len() > x[0] {
                        if s[x[0]] != '#' as u8 {
                            sum += res[idx_s + x[0] + 1][idx_x + 1];
                        }
                    } else if x.len() == 1 {
                        sum += 1;
                    }
                }
            }
            res[idx_s][idx_x] = sum;
        }
    }
    return res[0][0];
}

fn part_one() {
    let data = read();
    println!(
        "{}",
        data.iter()
            .map(|(s, x)| count(s.as_bytes(), &x))
            .fold(0, |x, y| x + y)
    );
}

fn part_two() {
    let data = read();
    let real_data = data
        .iter()
        .map(|(s, x)| {
            let mut real_s: String = String::new();
            let mut real_x: Vec<usize> = Vec::new();
            for i in 0..5 {
                real_s += s;
                if i != 4 {
                    real_s += "?";
                }
                real_x.extend_from_slice(x);
            }
            (real_s, real_x)
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        real_data
            .iter()
            .map(|(s, x)| count(s.as_bytes(), &x))
            .fold(0, |x, y| x + y)
    );
}
