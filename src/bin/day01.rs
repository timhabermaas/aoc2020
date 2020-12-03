use std::collections::HashSet;
use std::fs::read_to_string;

fn puzzle_1(numbers: &Vec<i32>, haystack: &HashSet<i32>) -> i32 {
    for n in numbers {
        let needle = 2020 - n;
        if haystack.contains(&needle) {
            return needle * n;
        }
    }
    return 0;
}

fn puzzle_2(numbers: &Vec<i32>, haystack: &HashSet<i32>) -> i32 {
    for a in numbers {
        for b in numbers {
            if haystack.contains(&(2020 - a - b)) {
                return a * b * (2020 - a - b);
            }
        }
    }
    return 0;
}

fn main() {
    let line_iter = read_to_string("./inputs/day01.txt").expect("file not found");
    let numbers = line_iter
        .lines()
        .map(|x| x.parse::<i32>().expect("not a valid number"));

    let haystack = numbers.clone().collect::<HashSet<_>>();

    // Result: 866436
    println!("{}", puzzle_1(&numbers.clone().collect(), &haystack));

    // Result: 276650720
    println!("{:?}", puzzle_2(&numbers.collect(), &haystack));
}
