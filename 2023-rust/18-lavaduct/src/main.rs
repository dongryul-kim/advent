fn main() {
    part_two();
}

fn read() -> Vec<(u8, usize, [u8; 6])> {
    std::io::stdin()
        .lines()
        .map(|l| {
            let l = l.unwrap();
            let mut it = l.split_ascii_whitespace();
            (
                it.next().unwrap().as_bytes()[0],
                it.next().unwrap().parse::<usize>().unwrap(),
                (it.next().unwrap().as_bytes()[2..8]).try_into().unwrap(),
            )
        })
        .collect()
}

fn calculate(data: &Vec<(u8, usize)>) -> usize {
    let mut row = 0;
    let mut area = 0;
    let mut len = 0;
    for (dir, step) in data {
        let step = *step as isize;
        len += step;
        match dir {
            b'R' => {
                area -= row * step;
            }
            b'L' => {
                area += row * step;
            }
            b'U' => {
                row -= step;
            }
            b'D' => {
                row += step;
            }
            _ => std::unreachable!(),
        }
    }
    (area.abs() + (len / 2) + 1) as usize
}

fn part_one() {
    let data = read();
    let data = data.iter().map(|&(a, b, _)| (a, b)).collect();
    println!("{}", calculate(&data));
}

fn part_two() {
    let data = read();
    let data = data
        .iter()
        .map(|&(_, _, s)| {
            (
                match s[5] {
                    b'0' => b'R',
                    b'1' => b'D',
                    b'2' => b'L',
                    b'3' => b'U',
                    _ => 0,
                },
                usize::from_str_radix(std::str::from_utf8(&s[0..5]).unwrap(), 16).unwrap(),
            )
        })
        .collect();
    println!("{}", calculate(&data));
}
