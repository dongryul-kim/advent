fn main() {
    part_two();
}

fn read() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect()
}

fn dfs(data: &Vec<Vec<u8>>, ene: &mut Vec<Vec<[bool; 4]>>, i: isize, j: isize, dir: usize) {
    if i < 0
        || i >= data.len() as isize
        || j < 0
        || j >= data[0].len() as isize
        || ene[i as usize][j as usize][dir]
    {
        return;
    }
    ene[i as usize][j as usize][dir] = true;
    match (dir, data[i as usize][j as usize]) {
        (0, b'.') | (0, b'-') | (1, b'/') | (3, b'\\') => {
            dfs(data, ene, i, j + 1, 0);
        },
        (0, b'/') | (1, b'.') | (1, b'|') | (2, b'\\') => {
            dfs(data, ene, i - 1, j, 1);
        },
        (1, b'\\') | (2, b'.') | (2, b'-') | (3, b'/') => {
            dfs(data, ene, i, j-1, 2);
        },
        (0, b'\\') | (2, b'/') | (3, b'.') | (3, b'|') => {
            dfs(data, ene, i + 1, j, 3);
        },
        (0, b'|') | (2, b'|') => {
            dfs(data, ene, i + 1, j, 3);
            dfs(data, ene, i - 1, j, 1);
        },
        (1, b'-') | (3, b'-') => {
            dfs(data, ene, i, j-1, 2);
            dfs(data, ene, i, j+1, 0);
        },
        _ => std::unreachable!(),
    }
}

fn num_energized(data: &Vec<Vec<u8>>, i: usize, j: usize, dir: usize) -> usize {
    // right, up, left, down
    let mut energized = vec![vec![[false; 4]; data[0].len()]; data.len()];
    dfs(&data, &mut energized, i as isize, j as isize, dir);
    (0..data.len()).map(|x| (0..data[0].len()).filter(|&y| energized[x][y] != [false;4]).count()).sum::<usize>()
}

fn part_one() {
    let data = read();
    println!("{}", num_energized(&data, 0, 0, 0));
}

fn part_two() {
    let data = read();
    let mut max = 0;
    for i in 0..data.len() {
        max = std::cmp::max(max, num_energized(&data, i, 0, 0));
        max = std::cmp::max(max, num_energized(&data, i, data[0].len()-1, 2));
    }
    for j in 0..data[0].len() {
        max = std::cmp::max(max, num_energized(&data, 0, j, 3));
        max = std::cmp::max(max, num_energized(&data, data.len()-1, j, 1));
    }
    println!("{max}");
}
