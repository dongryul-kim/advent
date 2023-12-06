fn main() {
    part_two();
}

fn read() -> (Vec<usize>, Vec<usize>) {
    let mut input = std::io::stdin().lines();
    let time: Vec<usize> = input
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let dist: Vec<usize> = input
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    return (time, dist);
}

fn possibilities(time: usize, dist: usize) -> usize {
    let mut cnt = 0;
    let mut left = 0;
    let mut right = time / 2;
    while left < right {
        let piv = (left + right) / 2;
        if piv * (time - piv) > dist {
            right = piv;
        } else {
            left = piv + 1;
        }
    }
    return time - 2 * left + 1;
}

fn part_one() {
    let (time, dist) = read();
    let prod = std::iter::zip(time, dist)
        .map(|x| match x {
            (t, d) => possibilities(t, d),
        })
        .reduce(|a, b| a * b)
        .unwrap();
    println!("{prod}");
}

fn part_two() {
    let (time, dist) = read();
    let time: usize = time
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()
        .parse()
        .unwrap();
    let dist: usize = dist
        .iter()
        .map(|x| x.to_string())
        .reduce(|a, b| a + &b)
        .unwrap()
        .parse()
        .unwrap();
    let ans = possibilities(time, dist);
    println!("{ans}");
}
