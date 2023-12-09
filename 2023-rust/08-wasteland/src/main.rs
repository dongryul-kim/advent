use std::collections::HashMap;

fn main() {
    part_two();
}

struct Data {
    instruction: String,
    map: HashMap<[u8; 3], ([u8; 3], [u8; 3], usize)>,
}

fn first_three(x: &[u8]) -> [u8; 3] {
    [x[0], x[1], x[2]]
}

fn read() -> Data {
    let mut lines = std::io::stdin().lines();
    let instruction: String = lines.next().unwrap().unwrap();
    lines.next();
    let mut map: HashMap<[u8; 3], ([u8; 3], [u8; 3], usize)> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let line = line.as_bytes();
        map.insert(
            first_three(&line[0..]),
            (first_three(&line[7..]), first_three(&line[12..]), 0),
        );
    }
    Data { instruction, map }
}

fn part_one() {
    let data = read();
    let mut cnt = 0;
    let mut pos = ['A' as u8; 3];
    let dest = ['Z' as u8; 3];
    for c in data.instruction.as_bytes().iter().cycle() {
        cnt += 1;
        match c {
            76 => pos = data.map[&pos].0,
            82 => pos = data.map[&pos].1,
            _ => {}
        };
        if pos == dest {
            break;
        }
    }
    println!("{cnt}");
}

fn analyze(data: &Data, pos: [u8; 3]) -> (usize, usize) {
    let mut cur = pos;
    let mut cnt = 0;
    let mut first: (usize, [u8; 3]) = (0, [0, 0, 0]);
    for c in data.instruction.as_bytes().iter().cycle() {
        cnt += 1;
        match c {
            76 => cur = data.map[&cur].0,
            82 => cur = data.map[&cur].1,
            _ => {}
        };
        if cur[2] == 'Z' as u8 {
            if first.0 == 0 {
                first = (cnt, cur);
            } else {
                assert!(cur == first.1);
                assert!((cnt - first.0) % data.instruction.len() == 0);
                assert!(cnt == 2 * first.0);
                return (first.0, cnt - first.0);
            }
        }
    }
    unreachable!();
}

fn part_two() {
    let data = read();
    let starts: Vec<_> = data
        .map
        .keys()
        .filter(|k| k[2] == 'A' as u8)
        .cloned()
        .map(|k| analyze(&data, k))
        .collect();
    let mut lcm = 1;
    for entry in starts {
        lcm = num::integer::lcm(lcm, entry.0);
    }
    println!("{lcm}");
}
