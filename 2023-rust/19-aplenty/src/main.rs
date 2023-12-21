fn main() {
    part_two();
}

enum Rule {
    Less(u8, usize, String),
    Greater(u8, usize, String),
    Else(String),
}

fn read() -> (
    std::collections::HashMap<String, Vec<Rule>>,
    Vec<[usize; 4]>,
) {
    let mut lines = std::io::stdin().lines();
    let mut map = std::collections::HashMap::<String, Vec<Rule>>::new();
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.is_empty() {
            break;
        }
        let name = line.split('{').next().unwrap().to_string();
        let line = line.split('{').nth(1).unwrap().to_string();
        let line = &line[..line.len() - 1];
        let rules = line
            .split(',')
            .map(|s| {
                if s.contains(':') {
                    let col = s.find(':').unwrap();
                    let name = s[col + 1..].to_string();
                    let num = s[2..col].parse::<usize>().unwrap();
                    let c = match s.as_bytes()[0] {
                        b'x' => 0,
                        b'm' => 1,
                        b'a' => 2,
                        b's' => 3,
                        _ => 0,
                    };
                    if s.as_bytes()[1] == b'<' {
                        Rule::Less(c, num, name)
                    } else {
                        Rule::Greater(c, num, name)
                    }
                } else {
                    Rule::Else(s.to_string())
                }
            })
            .collect();
        map.insert(name, rules);
    }
    let parts = lines
        .map(|line| {
            let line = line.unwrap();
            line[1..line.len() - 1]
                .split(',')
                .map(|s| s[2..].parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
                .try_into()
                .unwrap()
        })
        .collect();
    (map, parts)
}

fn apply(rules: &Vec<Rule>, part: &[usize; 4]) -> String {
    for rule in rules {
        match rule {
            Rule::Less(c, num, s) => {
                if part[*c as usize] < *num {
                    return s.to_string();
                }
            }
            Rule::Greater(c, num, s) => {
                if part[*c as usize] > *num {
                    return s.to_string();
                }
            }
            Rule::Else(s) => {
                return s.to_string();
            }
        }
    }
    std::unreachable!();
}

fn result(workflow: &std::collections::HashMap<String, Vec<Rule>>, part: &[usize; 4]) -> bool {
    let mut s = "in".to_string();
    loop {
        s = apply(&workflow[&s], part);
        if s == "A" {
            return true;
        } else if s == "R" {
            return false;
        }
    }
}

fn part_one() {
    let (workflow, parts) = read();
    let ans = parts
        .iter()
        .filter(|part| result(&workflow, part))
        .map(|part| part.iter().sum::<usize>())
        .sum::<usize>();
    println!("{ans}");
}

fn count(range: &[(usize, usize); 4]) -> usize {
    range.iter().map(|(a, b)| b + 1 - a).product::<usize>()
}

fn calculate(
    workflow: &std::collections::HashMap<String, Vec<Rule>>,
    mut range: [(usize, usize); 4],
    s: &str,
) -> usize {
    if s == "A" {
        return count(&range);
    } else if s == "R" {
        return 0;
    }
    let mut ans = 0;
    for rule in &workflow[s] {
        match rule {
            Rule::Less(c, num, s) => {
                let c = *c as usize;
                if *num <= range[c].0 {
                    continue;
                } else if range[c].1 < *num {
                    ans += calculate(workflow, range, s);
                    return ans;
                } else {
                    let orig = range[c].1;
                    range[c].1 = *num - 1;
                    ans += calculate(workflow, range, s);
                    range[c].1 = orig;
                    range[c].0 = *num;
                }
            }
            Rule::Greater(c, num, s) => {
                let c = *c as usize;
                if range[c].1 <= *num {
                    continue;
                } else if *num < range[c].0 {
                    ans += calculate(workflow, range, s);
                    return ans;
                } else {
                    let orig = range[c].0;
                    range[c].0 = *num + 1;
                    ans += calculate(workflow, range, s);
                    range[c].0 = orig;
                    range[c].1 = *num;
                }
            }
            Rule::Else(s) => {
                ans += calculate(workflow, range, s);
            }
        }
    }
    ans
}

fn part_two() {
    let (workflow, _) = read();
    let range = [(1, 4000); 4];
    println!("{}", calculate(&workflow, range, "in"));
}
