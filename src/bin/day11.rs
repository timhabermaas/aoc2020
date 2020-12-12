use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone)]
enum TileState {
    Occupied,
    Floor,
    Empty,
}

struct WaitingArea {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    state: TileState,
    neighbours: Vec<usize>,
    pos: (usize, usize),
}

fn clamp(number: usize, min: usize, max: usize) -> usize {
    if number < min {
        return min;
    } else if number > max {
        return max;
    }
    number
}

fn get_neighbours(position: usize, width: usize, height: usize) -> Vec<usize> {
    let y = position / width;
    let x = position - y * width;

    let mut result: HashSet<(usize, usize)> = vec![
        (clamp(x + 1, 0, width - 1), clamp(y + 1, 0, height - 1)),
        (x, clamp(y + 1, 0, height - 1)),
        (clamp(x + 1, 0, width - 1), y),
        (x.saturating_sub(1), y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x, y.saturating_sub(1)),
        (clamp(x + 1, 0, width - 1), y.saturating_sub(1)),
        (x.saturating_sub(1), clamp(y + 1, 0, height - 1)),
    ]
    .iter()
    .cloned()
    .collect();

    result.remove(&(x, y));
    println!("{:?}: {}", (x, y), result.len());
    println!("{:?}", result);

    result.iter().map(|(x, y)| y * width + x).collect()
}

fn neighbour_states(tile: &Tile, tiles: &[Tile]) -> Vec<TileState> {
    tile.neighbours
        .iter()
        .map(|n| &tiles[*n].state)
        .cloned()
        .collect()
}

fn simulate_rule(tiles: &[Tile]) -> Vec<Tile> {
    let result = tiles
        .iter()
        .map(|tile| match tile.state {
            TileState::Empty
                if neighbour_states(&tile, tiles)
                    .iter()
                    .all(|s| *s != TileState::Occupied) =>
            {
                Tile {
                    neighbours: tile.neighbours.clone(),
                    state: TileState::Occupied,
                    pos: tile.pos.clone(),
                }
            }
            TileState::Occupied
                if neighbour_states(&tile, tiles)
                    .iter()
                    .filter(|s| **s == TileState::Occupied)
                    .count()
                    >= 4 =>
            {
                Tile {
                    neighbours: tile.neighbours.clone(),
                    state: TileState::Empty,
                    pos: tile.pos.clone(),
                }
            }
            _ => tile.clone(),
        })
        .collect();
    result
}

fn simulate_rule_2(waiting_area: &WaitingArea) -> WaitingArea {
    let result = waiting_area
        .tiles
        .iter()
        .map(|tile| match tile.state {
            TileState::Empty if seen_seats(tile.pos, waiting_area).occupied == 0 => Tile {
                neighbours: tile.neighbours.clone(),
                state: TileState::Occupied,
                pos: tile.pos.clone(),
            },
            TileState::Occupied if seen_seats(tile.pos, waiting_area).occupied >= 5 => Tile {
                neighbours: tile.neighbours.clone(),
                state: TileState::Empty,
                pos: tile.pos.clone(),
            },
            _ => tile.clone(),
        })
        .collect();

    WaitingArea {
        width: waiting_area.width,
        height: waiting_area.height,
        tiles: result,
    }
}

fn walk_till_seat(
    pos: (usize, usize),
    dir: &(i32, i32),
    waiting_area: &WaitingArea,
) -> Option<TileState> {
    let WaitingArea {
        width,
        height,
        tiles,
    } = waiting_area;
    let mut new_pos: (i32, i32) = (pos.0 as i32, pos.1 as i32);
    loop {
        new_pos = (new_pos.0 + dir.0, new_pos.1 + dir.1);
        if new_pos.0 >= *width as i32 {
            return None;
        }
        if new_pos.0 < 0 || new_pos.1 < 0 {
            return None;
        }
        if new_pos.1 >= *height as i32 {
            return None;
        }
        if tiles[new_pos.1 as usize * width + new_pos.0 as usize].state == TileState::Occupied {
            return Some(TileState::Occupied);
        }
        if tiles[new_pos.1 as usize * width + new_pos.0 as usize].state == TileState::Empty {
            return Some(TileState::Empty);
        }
    }
}

struct SeatCounts {
    occupied: usize,
    free: usize,
}

/// Returns the amount of empty seats and occupied seats seen from `position`.
fn seen_seats(position: (usize, usize), waiting_area: &WaitingArea) -> SeatCounts {
    let directions = vec![
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, 0),
        (0, 1),
        (-1, 1),
        (1, -1),
    ];
    let mut result = SeatCounts {
        occupied: 0,
        free: 0,
    };
    for dir in directions {
        match walk_till_seat(position, &dir, waiting_area) {
            Some(TileState::Occupied) => result.occupied += 1,
            Some(TileState::Empty) => result.free += 1,
            _ => {}
        }
    }
    result
}

fn main() {
    let content = read_to_string("./inputs/day11.txt").expect("file not found");

    let character_count = content.chars().filter(|x| *x != '\n').count();
    let width = content.find('\n').unwrap();
    let height = character_count / width;

    let tiles: Vec<Tile> = content
        .chars()
        .filter(|x| *x != '\n')
        .enumerate()
        .map(|(i, x)| {
            let state = match x {
                'L' => TileState::Empty,
                '.' => TileState::Floor,
                '#' => TileState::Occupied,
                _ => panic!("unknown tile {}", x),
            };
            let neighbours = get_neighbours(i, width, height);
            let y = i / width;
            let x = i - y * width;
            Tile {
                state,
                neighbours,
                pos: (x, y),
            }
        })
        .collect();

    let waiting_area = WaitingArea {
        tiles: tiles,
        width,
        height,
    };

    // Task 1
    let mut old_tiles = waiting_area.tiles.clone();
    loop {
        let new_tiles = simulate_rule(&old_tiles);
        if new_tiles == old_tiles {
            println!("finish!");
            // Result: 2183
            println!(
                "{}",
                new_tiles
                    .iter()
                    .filter(|t| t.state == TileState::Occupied)
                    .count()
            );
            break;
        }
        old_tiles = new_tiles;
    }

    // Task 2
    let mut old_waiting_area = waiting_area;
    loop {
        let new_waiting_area = simulate_rule_2(&old_waiting_area);
        if new_waiting_area.tiles == old_waiting_area.tiles {
            println!("finish!");
            // Result: 1990
            println!(
                "{}",
                new_waiting_area
                    .tiles
                    .iter()
                    .filter(|t| t.state == TileState::Occupied)
                    .count()
            );
            break;
        }
        old_waiting_area = new_waiting_area;
    }
}
