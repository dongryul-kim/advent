fn main() {
    part_two();
}

fn part_one() {
    let mut sum = 0u32;
    for line in std::io::stdin().lines() {
        let word = line.unwrap();
        let first = word.find(|x: char| x.is_digit(10)).unwrap();
        let last = word.rfind(|x: char| x.is_digit(10)).unwrap();
        sum += 10 * word.chars().nth(first).unwrap().to_digit(10).unwrap();
        sum += word.chars().nth(last).unwrap().to_digit(10).unwrap();
    }
    println!("{sum}");
}

fn part_two() {
    let mut sum = 0;
    let numbers = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for line in std::io::stdin().lines() {
        let word = line.unwrap();

        let mut index = usize::MAX;
        let mut digit = 0;
        match word.find(|x: char| x.is_digit(10)) {
            Some(idx) => {
                index = idx;
                digit = word.chars().nth(idx).unwrap().to_digit(10).unwrap() as usize;
            }
            None => {}
        };
        for dig in 1..=9 {
            match word.find(numbers[dig]) {
                Some(idx) => {
                    if index > idx {
                        index = idx;
                        digit = dig;
                    }
                }
                None => {}
            }
        }
        sum += digit * 10;

        index = usize::MIN;
        match word.rfind(|x: char| x.is_digit(10)) {
            Some(idx) => {
                index = idx;
                digit = word.chars().nth(idx).unwrap().to_digit(10).unwrap() as usize;
            }
            None => {}
        };
        for dig in 1..=9 {
            match word.rfind(numbers[dig]) {
                Some(idx) => {
                    if index < idx {
                        index = idx;
                        digit = dig;
                    }
                }
                None => {}
            }
        }
        sum += digit;
    }
    println!("{sum}");
}
