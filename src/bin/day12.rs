use core::f64::consts::PI;
use std::fs::read_to_string;

#[derive(Debug)]
struct Ferry {
    pos: (i64, i64),
    dir: (i64, i64),
}

fn manhattan_distance(pos: &(i64, i64)) -> i64 {
    pos.0.abs() + pos.1.abs()
}

//static map: Vec<((i64, i64), i64)> = vec![((1, 0), 90)];
//
static deg_to_radian: f64 = PI / 180.0;

fn rotate_right((x, y): &(i64, i64), degree: i64) -> (i64, i64) {
    let deg_in_radian = degree as f64 * deg_to_radian;

    let t = (
        *x as f64 * deg_in_radian.cos() - *y as f64 * deg_in_radian.sin(),
        *x as f64 * deg_in_radian.sin() + *y as f64 * deg_in_radian.cos(),
    );

    //(t.0.round() as i64 + org_x, t.1.round() as i64 + org_y)
    (t.0.round() as i64, t.1.round() as i64)
}

fn rotate_left(dir: &(i64, i64), degree: i64) -> (i64, i64) {
    rotate_right(dir, 360 - degree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_right() {
        assert_eq!(rotate_right(&(1, 0), 90), (0, 1));
        assert_eq!(rotate_right(&(1, 0), 180), (-1, 0));
        assert_eq!(rotate_right(&(1, 0), 270), (0, -1));
    }

    #[test]
    fn test_rotate_left() {
        assert_eq!(rotate_left(&(1, 0), 90), (0, -1));
        assert_eq!(rotate_left(&(1, 0), 180), (-1, 0));
        assert_eq!(rotate_left(&(1, 0), 270), (0, 1));
    }
}

fn main() {
    let content = read_to_string("./inputs/day12.txt").expect("file not found");

    // Task 1
    let result = content.lines().fold(
        Ferry {
            pos: (0, 0),
            dir: (1, 0),
        },
        |Ferry { pos, dir }, instruction| {
            let command = &instruction.chars().nth(0).unwrap();
            let value: i64 = instruction[1..].parse().unwrap();
            match command {
                'N' => Ferry {
                    pos: (pos.0, pos.1 - value),
                    dir,
                },
                'S' => Ferry {
                    pos: (pos.0, pos.1 + value),
                    dir,
                },
                'E' => Ferry {
                    pos: (pos.0 + value, pos.1),
                    dir,
                },
                'W' => Ferry {
                    pos: (pos.0 - value, pos.1),
                    dir,
                },
                'L' => Ferry {
                    pos,
                    dir: rotate_left(&dir, value),
                },
                'R' => Ferry {
                    pos,
                    dir: rotate_right(&dir, value),
                },
                'F' => Ferry {
                    pos: (pos.0 + dir.0 * value, pos.1 + dir.1 * value),
                    dir,
                },
                _ => panic!("unknown command {}", command),
            }
        },
    );

    // Result: 1294
    println!("{:?}", manhattan_distance(&result.pos));

    // Task 2
    let result = content.lines().fold(
        Ferry {
            pos: (0, 0),
            dir: (10, -1),
        },
        |Ferry { pos, dir }, instruction| {
            let command = &instruction.chars().nth(0).unwrap();
            let value: i64 = instruction[1..].parse().unwrap();

            match command {
                'N' => Ferry {
                    pos,
                    dir: (dir.0, dir.1 - value),
                },
                'S' => Ferry {
                    pos,
                    dir: (dir.0, dir.1 + value),
                },
                'E' => Ferry {
                    pos,
                    dir: (dir.0 + value, dir.1),
                },
                'W' => Ferry {
                    pos,
                    dir: (dir.0 - value, dir.1),
                },
                'L' => Ferry {
                    pos,
                    dir: rotate_left(&dir, value),
                },
                'R' => Ferry {
                    pos,
                    dir: rotate_right(&dir, value),
                },
                'F' => Ferry {
                    pos: (pos.0 + dir.0 * value, pos.1 + dir.1 * value),
                    dir,
                },
                _ => panic!("unknown command {}", command),
            }
        },
    );

    // Result 20592
    println!("{:?}", manhattan_distance(&result.pos));
}
