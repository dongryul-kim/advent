fn main() {
    part_two()
}

struct Entry {
    hand: [char; 5],
    num: usize,
}

fn read() -> Vec<Entry> {
    let mut vec: Vec<Entry> = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut split = line.split_ascii_whitespace();
        vec.push(Entry {
            hand: split
                .next()
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .unwrap(),
            num: split.next().unwrap().parse().unwrap(),
        });
    }
    vec
}

fn char_to_num(x: char) -> usize {
    if '2' <= x && x <= '9' {
        (x as usize) - ('2' as usize)
    } else {
        match x {
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => 0,
        }
    }
}

fn char_to_num_two(x: char) -> usize {
    if '2' <= x && x <= '9' {
        (x as usize) - ('1' as usize)
    } else {
        match x {
            'T' => 9,
            'J' => 0,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => 0,
        }
    }
}

fn score(hand: &[char; 5]) -> usize {
    let mut hand_sort = hand.clone();
    hand_sort.sort();
    let mut groups: Vec<usize> = Vec::new();
    let mut cnt = 0;
    let mut cur: char = 0 as char;
    for card in hand_sort {
        if cur != card {
            groups.push(cnt);
            cnt = 1;
            cur = card;
        } else {
            cnt += 1;
        }
    }
    groups.push(cnt);
    groups.sort();
    let mut score = match groups[..] {
        [0, 5] => 6,
        [0, 1, 4] => 5,
        [0, 2, 3] => 4,
        [0, 1, 1, 3] => 3,
        [0, 1, 2, 2] => 2,
        [0, 1, 1, 1, 2] => 1,
        [0, 1, 1, 1, 1, 1] => 0,
        _ => 0,
    };
    for c in hand {
        score = 13 * score + char_to_num(*c);
    }
    score
}

fn score_two(hand: &[char; 5]) -> usize {
    let mut hand_sort = hand.clone();
    hand_sort.sort();
    let mut groups: Vec<usize> = Vec::new();
    let mut cnt = 0;
    let mut cur: char = 0 as char;
    let joker = hand_sort
        .iter()
        .filter(|x| **x == 'J')
        .collect::<Vec<_>>()
        .len();
    for card in hand_sort.iter().filter(|x| **x != 'J') {
        if cur != *card {
            groups.push(cnt);
            cnt = 1;
            cur = *card;
        } else {
            cnt += 1;
        }
    }
    groups.push(cnt);
    groups.sort();
    let glen = groups.len();
    groups[glen - 1] += joker;
    let mut score = match groups[..] {
        [0, 5] => 6,
        [0, 1, 4] => 5,
        [0, 2, 3] => 4,
        [0, 1, 1, 3] => 3,
        [0, 1, 2, 2] => 2,
        [0, 1, 1, 1, 2] => 1,
        [0, 1, 1, 1, 1, 1] => 0,
        [5] => 6,
        _ => 0,
    };
    for c in hand {
        score = 13 * score + char_to_num_two(*c);
    }
    score
}

fn part_one() {
    let data = read();
    let mut scores: Vec<_> = data.iter().map(|e| (score(&e.hand), e.num)).collect();
    scores.sort();
    let sum = scores
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1) * v.1)
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{sum}");
}

fn part_two() {
    let data = read();
    let mut scores: Vec<_> = data.iter().map(|e| (score_two(&e.hand), e.num)).collect();
    scores.sort();
    let sum = scores
        .iter()
        .enumerate()
        .map(|(i, v)| (i + 1) * v.1)
        .reduce(|a, b| a + b)
        .unwrap();
    println!("{sum}");
}
