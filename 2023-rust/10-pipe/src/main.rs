fn main() {
    part_two();
}

fn read() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().iter().map(|&x| x).collect())
        .collect()
}

fn find_start(map: &Vec<Vec<u8>>) -> (usize, usize) {
    for (i, line) in map.iter().enumerate() {
        for (j, &x) in line.iter().enumerate() {
            if x == 'S' as u8 {
                return (i, j);
            }
        }
    }
    std::unreachable!();
}

#[derive(Clone)]
enum Direction {
    Up = 0,
    Down = 3,
    Left = 1,
    Right = 2,
}

fn next_dir(dir: Direction, pipe: u8) -> Direction {
    let sum = match pipe as char {
        '|' => (Direction::Down as u8) + (Direction::Up as u8),
        '-' => (Direction::Left as u8) + (Direction::Right as u8),
        'J' => (Direction::Left as u8) + (Direction::Up as u8),
        'L' => (Direction::Right as u8) + (Direction::Up as u8),
        'F' => (Direction::Down as u8) + (Direction::Right as u8),
        '7' => (Direction::Down as u8) + (Direction::Left as u8),
        _ => std::unreachable!(),
    };
    match sum + (dir as u8) - 3 {
        0 => Direction::Up,
        1 => Direction::Left,
        2 => Direction::Right,
        3 => Direction::Down,
        _ => std::unreachable!(),
    }
}

fn part_one() {
    let map = read();
    let init_pos = find_start(&map);
    let mut pos = init_pos;
    let mut dir = Direction::Left;
    let mut cnt = 0;
    loop {
        cnt += 1;
        match dir {
            Direction::Up => pos.0 -= 1,
            Direction::Down => pos.0 += 1,
            Direction::Left => pos.1 -= 1,
            Direction::Right => pos.1 += 1,
        }
        if map[pos.0][pos.1] == 'S' as u8 {
            break;
        }
        dir = next_dir(dir, map[pos.0][pos.1]);
    }
    println!("{}", cnt / 2);
}

fn part_two() {
    let map = read();
    let init_pos = find_start(&map);
    let mut pos = init_pos;
    let mut dir = Direction::Left;
    let mut cnt = 0;
    let mut sum: i64 = 0;
    loop {
        cnt += 1;
        match dir {
            Direction::Up => pos.0 -= 1,
            Direction::Down => pos.0 += 1,
            Direction::Left => pos.1 -= 1,
            Direction::Right => pos.1 += 1,
        }
        match dir {
            Direction::Up => sum += pos.1 as i64,
            Direction::Down => sum -= pos.1 as i64,
            Direction::Left => sum -= pos.0 as i64,
            Direction::Right => sum += pos.0 as i64,
        }
        if map[pos.0][pos.1] == 'S' as u8 {
            break;
        }
        dir = next_dir(dir, map[pos.0][pos.1]);
    }
    println!("{}", (sum.abs() + 2 - (cnt as i64)) / 2);
}
