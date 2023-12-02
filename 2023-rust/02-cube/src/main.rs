fn main() {
    part_two();
}

fn part_one() {
    let mut cnt = 0;
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        cnt += 1;

        let (_, line) = line.split_at(line.find(':').unwrap() + 2);
        let mut flag = true;
        for game in line.split("; ") {
            for entry in game.split(", ") {
                let (num, color) = entry.split_at(entry.find(' ').unwrap());
                let num: usize = num.parse::<usize>().unwrap();
                flag = flag
                    && match color.trim() {
                        "red" => num <= 12,
                        "green" => num <= 13,
                        "blue" => num <= 14,
                        _ => true,
                    };
            }
        }
        if flag {
            sum += cnt;
        }
    }
    println!("{sum}");
}

fn part_two() {
    let mut sum = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        let (_, line) = line.split_at(line.find(':').unwrap() + 2);
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for game in line.split("; ") {
            for entry in game.split(", ") {
                let (num, color) = entry.split_at(entry.find(' ').unwrap());
                let num: usize = num.parse::<usize>().unwrap();
                match color.trim() {
                    "red" => {
                        min_red = std::cmp::max(min_red, num);
                    }
                    "green" => {
                        min_green = std::cmp::max(min_green, num);
                    }
                    "blue" => {
                        min_blue = std::cmp::max(min_blue, num);
                    }
                    _ => {}
                };
            }
        }
        sum += min_red * min_green * min_blue;
    }
    println!("{sum}");
}
