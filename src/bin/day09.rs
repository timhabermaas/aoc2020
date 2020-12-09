use std::fs::read_to_string;

fn is_sum_of_numbers(needle: u64, haystack: &Vec<u64>) -> bool {
    for number in haystack {
        if *number <= needle {
            let x = needle - number;
            if haystack.contains(&x) && x != needle {
                return true;
            }
        }
    }
    false
}

fn main() {
    let content = read_to_string("./inputs/day09.txt").expect("file not found");
    let numbers: Vec<u64> = content
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut latest_25_numbers: Vec<u64> = numbers[0..25].to_owned();

    // Task 1
    for number in &numbers[25..] {
        if !is_sum_of_numbers(*number, &latest_25_numbers) {
            println!("is not sum");
            // Result: 1124361034
            println!("{}", number);
            break;
        }

        latest_25_numbers.remove(0);
        latest_25_numbers.push(*number);
    }

    // Task 2
    //
    // Find a sequence of numbers which sum up to 1124361034.
    // We can keep a window and keep summing it up. If the sum matches: end. If the sum is too big,
    // remove the first entry.
    //
    let target_sum: u64 = 1124361034;

    // Prefill window with first two elements.
    let mut summands_window: Vec<u64> = numbers[0..2].to_owned();

    for number in &numbers[2..] {
        let mut sum: u64 = summands_window.iter().sum();
        if sum == target_sum {
            println!("found sum");
            println!("{:?}", summands_window);
            summands_window.sort();
            let min = summands_window.iter().nth(0).unwrap();
            let max = summands_window.last().unwrap();
            // Result: 129444555
            println!("min: {}, max: {}, sum: {}", min, max, min + max);
            break;
        }
        summands_window.push(*number);
        sum = summands_window.iter().sum();
        // Ensure summands_window has always >= 2 elements.
        while sum > target_sum && summands_window.len() > 2 {
            summands_window.remove(0);
            sum = summands_window.iter().sum();
        }
    }
}
