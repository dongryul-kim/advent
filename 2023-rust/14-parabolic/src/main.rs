use std::collections::HashMap;

fn main() {
    part_two();
}

fn read() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect()
}

fn tilt_north(map: &mut Vec<Vec<u8>>) {
    for i in 1..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == b'O' {
                let mut piv = i;
                while piv > 0 && map[piv - 1][j] == b'.' {
                    piv -= 1;
                }
                map[i][j] = b'.';
                map[piv][j] = b'O';
            }
        }
    }
}

fn rotate_clockwise(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut res = vec![vec![b'.'; map.len()]; map[0].len()];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            res[j][map.len() - i - 1] = map[i][j];
        }
    }
    res
}

fn tilt_cycle(map: &mut Vec<Vec<u8>>) {
    for _ in 0..4 {
        tilt_north(map);
        *map = rotate_clockwise(map);
    }
}

fn ans(map: &Vec<Vec<u8>>) -> usize {
    (0..map.len())
        .map(|i| (map.len() - i) * ((0..map[0].len()).filter(|&j| map[i][j] == b'O').count()))
        .sum::<usize>()
}

fn part_one() {
    let mut data = read();
    tilt_north(&mut data);
    println!("{}", ans(&data));
}

fn part_two() {
    let target = 1_000_000_000;
    let mut data = read();
    let mut hash_map: HashMap<Vec<Vec<u8>>, usize> = HashMap::new();
    let mut anss: Vec<usize> = Vec::new();
    for cnt in 0.. {
        if hash_map.contains_key(&data) {
            let prev_cnt = hash_map[&data];
            println!(
                "{}",
                anss[prev_cnt + ((target - prev_cnt) % (cnt - prev_cnt))]
            );
            return;
        } else {
            let ans = ans(&data);
            hash_map.insert(data.clone(), cnt);
            anss.push(ans);
        }
        tilt_cycle(&mut data);
    }
}
