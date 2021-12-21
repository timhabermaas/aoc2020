use std::collections::HashMap;
use std::fs::read_to_string;

fn parse(input: &str) -> Vec<u64> {
    input.lines().map(str::parse).map(Result::unwrap).collect()
}

fn main() {
    let content = read_to_string("./inputs/day10.txt").expect("file not found");

    let mut adapters = parse(&content);

    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters[adapters.len() - 1] + 3);

    let adapters = adapters;

    let mut differences: (usize, usize, usize) = (0, 0, 0);

    for x in adapters.windows(2) {
        if let [first, second] = x {
            let difference = second - first;
            if difference == 1 {
                differences = (differences.0 + 1, differences.1, differences.2);
            }
            if difference == 3 {
                differences = (differences.0, differences.1, differences.2 + 1);
            }
        }
    }

    let (ones, _, threes) = differences;

    // Result: 2592
    println!("Part 1: {}", ones * threes);

    println!("Part 2: {}", part_2(&adapters));
}

fn part_2(adapters: &[u64]) -> usize {
    // count of possibilities to get from index to end
    let mut dp: HashMap<usize, usize> = HashMap::new();
    dp.insert(adapters.len() - 1, 1);

    for i in (0..adapters.len() - 1).rev() {
        let current_j = adapters[i];
        let mut possibilities = 0;

        for step in 1..=3 {
            if i + step > adapters.len() - 1 {
                continue;
            }

            if adapters[i + step] - current_j <= 3 {
                possibilities += dp[&(i + step)]
            }
        }
        dp.insert(i, possibilities);
    }

    dp[&0]
}
