use itertools::Itertools;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

fn parse_cards(input: &str) -> VecDeque<u32> {
    let mut lines = input.lines();
    lines.next().unwrap();

    lines.map(|l| l.parse().unwrap()).collect()
}

fn main() {
    let input = read_to_string("./inputs/day22.txt").expect("file not found");
    let (p1, p2) = input.split("\n\n").collect_tuple().unwrap();

    let mut p1 = parse_cards(p1);
    let mut p2 = parse_cards(p2);

    while play(&mut p1, &mut p2) == 0 {}

    println!("Part 1: {}", std::cmp::max(score(&p1), score(&p2)));

    let (p1, p2) = input.split("\n\n").collect_tuple().unwrap();

    let mut p1 = parse_cards(p1);
    let mut p2 = parse_cards(p2);

    play_2(&mut p1, &mut p2, HashSet::new(), HashSet::new(), 1);
    println!("Part 2: {}", std::cmp::max(score(&p1), score(&p2)));
}

fn play(deck1: &mut VecDeque<u32>, deck2: &mut VecDeque<u32>) -> u8 {
    if deck1.is_empty() {
        return 1;
    }
    if deck2.is_empty() {
        return 2;
    }

    let c1 = deck1.pop_front().unwrap();
    let c2 = deck2.pop_front().unwrap();

    if c1 > c2 {
        deck1.push_back(c1);
        deck1.push_back(c2);
    } else {
        deck2.push_back(c2);
        deck2.push_back(c1);
    }

    0
}

// TODO: Add HashSet
fn play_2(
    deck1: &mut VecDeque<u32>,
    deck2: &mut VecDeque<u32>,
    mut seen1: HashSet<VecDeque<u32>>,
    mut seen2: HashSet<VecDeque<u32>>,
    game: usize,
) -> u8 {
    let mut round = 1;
    loop {
        if seen1.contains(&deck1) || seen2.contains(&deck2) {
            return 1;
        }

        seen1.insert(deck1.clone());
        seen2.insert(deck2.clone());

        if deck1.is_empty() {
            return 2;
        }
        if deck2.is_empty() {
            return 1;
        }

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        let winner;

        if deck1.len() as u32 >= c1 && deck2.len() as u32 >= c2 {
            let mut d1: VecDeque<u32> = deck1.iter().take(c1 as usize).copied().collect();
            let mut d2: VecDeque<u32> = deck2.iter().take(c2 as usize).copied().collect();
            winner = play_2(&mut d1, &mut d2, HashSet::new(), HashSet::new(), game + 1);
        } else {
            if c1 > c2 {
                winner = 1;
            } else {
                winner = 2;
            }
        }

        if winner == 1 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }

        //println!("Player {} wins round {} of game {}", winner, round, game);

        round += 1;
    }
}

fn score(cards: &VecDeque<u32>) -> u32 {
    cards
        .iter()
        .rev()
        .enumerate()
        .map(|(i, x)| (i as u32 + 1) * x)
        .sum()
}
