fn main() {
    part_two();
}

fn read() -> Vec<Vec<Vec<u8>>> {
    let mut data: Vec<Vec<Vec<u8>>> = Vec::new();
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let row: Vec<u8> = line.as_bytes().to_vec();
        if !row.is_empty() {
            map.push(row);
        } else {
            data.push(map.clone());
            map.clear();
        }
    }
    data.push(map.clone());
    data
}

fn find_reflection(map: &Vec<Vec<u8>>, target_err: usize) -> usize {
    for hor in 1..map.len() {
        let err = (0..std::cmp::min(hor, map.len() - hor))
            .map(|i| {
                (0..map[0].len())
                    .map(|j| match map[hor + i][j] == map[hor - i - 1][j] {
                        true => 0,
                        false => 1,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        if err == target_err {
            return 100 * hor;
        }
    }
    for ver in 1..map[0].len() {
        let err = (0..std::cmp::min(ver, map[0].len() - ver))
            .map(|j| {
                (0..map.len())
                    .map(|i| match map[i][ver + j] == map[i][ver - j - 1] {
                        true => 0,
                        false => 1,
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        if err == target_err {
            return ver;
        }
    }
    std::unreachable!();
}

fn part_one() {
    let data = read();
    println!(
        "{}",
        data.iter()
            .map(|map| find_reflection(map, 0))
            .sum::<usize>()
    );
}

fn part_two() {
    let data = read();
    println!(
        "{}",
        data.iter()
            .map(|map| find_reflection(map, 1))
            .sum::<usize>()
    );
}
