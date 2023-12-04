fn main() {
    part_two();
}

fn read() -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut res: Vec<(Vec<usize>, Vec<usize>)> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let line = line.split(": ").nth(1).unwrap();
        let winning = line
            .split(" | ")
            .nth(0)
            .unwrap()
            .split_ascii_whitespace()
            .map(|s: &str| s.parse::<usize>().unwrap())
            .collect();
        let nums = line
            .split(" | ")
            .nth(1)
            .unwrap()
            .split_ascii_whitespace()
            .map(|s: &str| s.parse::<usize>().unwrap())
            .collect();
        res.push((winning, nums));
    }
    return res;
}

fn count_intersection(data: &Vec<(Vec<usize>, Vec<usize>)>) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    for (winning, nums) in data {
        let mut winset = std::collections::BTreeSet::<usize>::new();
        for x in winning {
            winset.insert(*x);
        }
        let cnt = nums.iter().filter(|x| winset.contains(x)).count();
        res.push(cnt);
    }
    return res;
}

fn part_one() {
    let data = read();
    let cnts = count_intersection(&data);
    let mut score = 0;
    for cnt in cnts {
        if cnt > 0 {
            score += 1 << (cnt - 1);
        }
    }
    println!("{score}");
}

fn part_two() {
    let data = read();
    let cnts = count_intersection(&data);
    let mut num_card: Vec<usize> = Vec::new();
    num_card.resize(cnts.len(), 1);
    for i in 0..cnts.len() {
        for j in 0..cnts[i] {
            num_card[i+j+1] += num_card[i];
        }
    }
    let total = num_card.iter().fold(0, |acc, e| acc+e);
    println!("{total}");
}
