use itertools::Itertools;
use itertools::MinMaxResult;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Dir {
    fn walk(&self, pos: (i32, i32)) -> (i32, i32) {
        use Dir::*;

        match self {
            E => (pos.0 + 1, pos.1),
            SE => (pos.0, pos.1 + 1),
            SW => (pos.0 - 1, pos.1 + 1),
            W => (pos.0 - 1, pos.1),
            NW => (pos.0, pos.1 - 1),
            NE => (pos.0 + 1, pos.1 - 1),
        }
    }

    fn neighbours(pos: (i32, i32)) -> HashSet<(i32, i32)> {
        use Dir::*;

        vec![
            E.walk(pos),
            SE.walk(pos),
            SW.walk(pos),
            W.walk(pos),
            NW.walk(pos),
            NE.walk(pos),
        ]
        .iter()
        .copied()
        .collect()
    }
}

fn parse_line(line: &str) -> Vec<Dir> {
    let chars = line.chars().collect::<Vec<_>>();

    let mut result = vec![];

    let mut prev: Option<char> = None;

    for i in 0..chars.len() {
        let last_prev = prev.take();

        match (last_prev, chars[i]) {
            (Some('s'), 'e') => result.push(Dir::SE),
            (Some('s'), 'w') => result.push(Dir::SW),
            (Some('n'), 'e') => result.push(Dir::NE),
            (Some('n'), 'w') => result.push(Dir::NW),
            (None, 'e') => result.push(Dir::E),
            (None, 'w') => result.push(Dir::W),
            (None, 's') => prev = Some('s'),
            (None, 'n') => prev = Some('n'),
            _ => panic!("input fail"),
        }
    }

    result
}

fn main() {
    let input = read_to_string("./inputs/day24.txt").expect("file not found");
    let dirs: Vec<Vec<Dir>> = input.lines().map(|l| parse_line(l)).collect();

    // Using skewed coordinates, see https://www.redblobgames.com/grids/hexagons/#coordinates
    let mut floor: HashSet<(i32, i32)> = HashSet::new();

    for row in dirs {
        let tile = row.iter().fold((0, 0), |pos, dir| dir.walk(pos));

        if floor.contains(&tile) {
            floor.remove(&tile);
        } else {
            floor.insert(tile);
        }
    }

    println!("Part 1: {}", floor.len());

    for _ in 1..=100 {
        let mut new_floor = floor.clone();

        let (min_x, max_x) = match floor.iter().map(|(x, _)| x).minmax() {
            MinMaxResult::MinMax(a, b) => (*a, *b),
            _ => panic!("fooo"),
        };
        let (min_y, max_y) = match floor.iter().map(|(_, y)| y).minmax() {
            MinMaxResult::MinMax(a, b) => (*a, *b),
            _ => panic!("fooo"),
        };

        for (x, y) in (min_x - 2..max_x + 2).cartesian_product(min_y - 2..max_y + 2) {
            let adj_blacks: usize = Dir::neighbours((x, y)).intersection(&floor).count();

            // black
            if floor.contains(&(x, y)) && (adj_blacks == 0 || adj_blacks > 2) {
                new_floor.remove(&(x, y));
            // white
            } else if adj_blacks == 2 {
                new_floor.insert((x, y));
            }
        }

        floor = new_floor;
    }

    println!("Part 2: {}", floor.len());
}
