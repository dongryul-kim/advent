fn main() {
    part_two();
}

#[derive(Clone)]
struct Entry {
    dest: usize,
    source: usize,
    len: usize,
}

struct Data {
    seed: Vec<usize>,
    maps: Vec<Vec<Entry>>,
}

fn read() -> Data {
    let mut line = std::io::stdin().lines();
    let seed: Vec<usize> = line
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s: &str| s.parse::<usize>().unwrap())
        .collect();
    line.next();

    let mut maps: Vec<Vec<Entry>> = Vec::new();
    let mut map_temp: Vec<Entry> = Vec::new();
    for line in line {
        let line = line.unwrap();
        if line.len() == 0 {
            // map_temp.sort_by_key(|k| k.source);
            maps.push(map_temp.clone());
            map_temp.clear();
            continue;
        } else if line.contains(":") {
            continue;
        }
        let v: Vec<_> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        map_temp.push(Entry {
            dest: v[0],
            source: v[1],
            len: v[2],
        });
    }
    maps.push(map_temp);
    return Data { seed, maps };
}

fn min_option(a: Option<usize>, b: Option<usize>) -> Option<usize> {
    match a {
        Some(x) => match b {
            Some(y) => Some(std::cmp::min(x, y)),
            None => Some(x),
        },
        None => b,
    }
}

fn apply(map: &Vec<Entry>, source: usize) -> (usize, Option<usize>) {
    let mut stretch: Option<usize> = None;
    for entry in map {
        if entry.source > source {
            stretch = min_option(stretch, Some(entry.source - source));
        }
        if entry.source + entry.len > source {
            stretch = min_option(stretch, Some(entry.source + entry.len - source));
        }
    }
    for entry in map {
        if entry.source <= source && entry.source + entry.len > source {
            return (entry.dest + (source - entry.source), stretch);
        }
    }
    return (source, stretch);
}

fn part_one() {
    let data = read();
    let mut min = usize::MAX;
    for seed in &data.seed {
        let mut cur = *seed;
        for map in &data.maps {
            (cur, _) = apply(map, cur);
        }
        if min > cur {
            min = cur;
        }
    }
    println!("{min}");
}

fn part_two() {
    let data = read();
    let mut min = usize::MAX;
    for i in 0..(data.seed.len() / 2) {
        let start = data.seed[2 * i];
        let len = data.seed[2 * i + 1];
        let mut piv = start;
        while piv < start + len {
            let mut cur = piv;
            let mut stretch: Option<usize> = None;
            for map in &data.maps {
                let (next_cur, stretch_cur) = apply(map, cur);
                cur = next_cur;
                stretch = min_option(stretch, stretch_cur);
            }
            if min > cur {
                min = cur;
            }
            match stretch {
                Some(x) => piv += x,
                None => break,
            }
        }
    }
    println!("{min}");
}
