fn main() {
    part_two();
}

fn read() -> Vec<Vec<u8>> {
    std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|x| x.as_bytes().to_vec())
        .collect()
}

fn hash(s: &[u8]) -> usize {
    let mut res = 0;
    for c in s {
        res += *c as usize;
        res *= 17;
        res %= 256;
    }
    res
}

fn part_one() {
    let data = read();
    println!("{}", data.iter().map(|s| hash(s)).sum::<usize>());
}

fn part_two() {
    let data = read();
    let mut map: [Vec<(Vec<u8>, usize)>; 256] = std::array::from_fn(|_| vec![]);
    for s in data {
        match s[s.len() - 1] {
            b'-' => {
                let s = &s[0..s.len() - 1];
                let vec = &mut map[hash(s)];
                for i in 0..vec.len() {
                    if vec[i].0 == s {
                        vec.remove(i);
                        break;
                    }
                }
            }
            _ => {
                let num = (s[s.len() - 1] - b'0') as usize;
                let s = &s[0..s.len() - 2];
                let vec = &mut map[hash(s)];
                for i in 0..vec.len() + 1 {
                    if i == vec.len() {
                        vec.push((s.to_vec(), num));
                    } else if vec[i].0 == s {
                        vec[i].1 = num;
                        break;
                    }
                }
            }
        }
    }
    let ans = (0..256)
        .map(|b| {
            (b + 1) * (map[b].iter().enumerate().map(|(cnt, v)| (cnt + 1) * v.1)).sum::<usize>()
        })
        .sum::<usize>();
    println!("{ans}");
}
