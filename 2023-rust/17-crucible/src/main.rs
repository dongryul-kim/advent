fn main() {
    part_two();
}

fn read() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().as_bytes().iter().map(|&c| c - b'0').collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct Entry {
    dist: usize,
    pos: (usize, usize),
    dir: usize,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.pos.cmp(&other.pos))
            .then_with(|| self.dir.cmp(&other.dir))
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(min: isize, max: isize) {
    let data = read();
    // right, up, left, down
    let mut mins = vec![vec![[0; 4]; data[0].len()]; data.len()];
    let mut heap = std::collections::BinaryHeap::<Entry>::new();
    heap.push(Entry {
        dist: 0,
        pos: (0, 0),
        dir: 0,
    });
    heap.push(Entry {
        dist: 0,
        pos: (0, 0),
        dir: 3,
    });
    loop {
        let entry = heap.pop().unwrap();
        if mins[entry.pos.0][entry.pos.1][entry.dir] > 0 {
            continue;
        }
        mins[entry.pos.0][entry.pos.1][entry.dir] = entry.dist;
        if entry.pos == (data.len() - 1, data[0].len() - 1) {
            println!("{}", entry.dist);
            return;
        }
        for new_dir in [(entry.dir + 1) % 4, (entry.dir + 3) % 4] {
            let disp: (isize, isize) = match new_dir {
                0 => (0, 1),
                1 => (-1, 0),
                2 => (0, -1),
                3 => (1, 0),
                _ => std::unreachable!(),
            };
            let mut new_dist = entry.dist;
            for step in 1..=max {
                let new_pos = (
                    entry.pos.0 as isize + step * disp.0,
                    entry.pos.1 as isize + step * disp.1,
                );
                if new_pos.0 < 0
                    || new_pos.0 >= data.len() as isize
                    || new_pos.1 < 0
                    || new_pos.1 >= data[0].len() as isize
                {
                    break;
                }
                let pos = (new_pos.0 as usize, new_pos.1 as usize);
                new_dist += data[pos.0][pos.1] as usize;
                if step < min {
                    continue;
                }
                heap.push(Entry {
                    dist: new_dist,
                    pos,
                    dir: new_dir,
                });
            }
        }
    }
}

fn part_one() {
    solve(1, 3);
}

fn part_two() {
    solve(4, 10);
}
