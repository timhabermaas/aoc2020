use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let content = read_to_string("./inputs/day06.txt").expect("file not found");
    let groups: Vec<&str> = content.split("\n\n").collect();

    // Task 1:
    let mut count = 0;
    for group in groups.clone() {
        let questions: HashSet<char> = group.chars().filter(|c| *c != '\n').collect();
        count += questions.len();
    }
    // Result: 7110
    println!("{}", count);

    // Task 2:
    let mut count = 0;
    for group in groups {
        let question_sets = group
            .lines()
            .map(|l| l.chars().collect::<HashSet<char>>())
            .collect::<Vec<_>>();

        let mut group_questions = question_sets[0].clone();
        for questions in question_sets {
            group_questions = &group_questions & &questions;
        }

        count += group_questions.len();
    }
    // Result: 3628
    println!("{}", count);
}
