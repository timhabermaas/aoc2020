use std::fs::read_to_string;

#[derive(PartialEq)]
enum Object {
    Tree,
    Snow,
}

struct Vec2D {
    x: usize,
    y: usize,
}
impl Vec2D {
    pub fn new(x: usize, y: usize) -> Self {
        Vec2D { x, y }
    }

    pub fn add(&mut self, other: &Vec2D) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

fn check_slope(buffer: &Vec<Object>, width: usize, height: usize, direction: &Vec2D) -> usize {
    let mut position = Vec2D::new(0, 0);
    let mut tree_count = 0;
    loop {
        // Move first to skip top-left;
        position.add(&direction);

        if position.y >= height {
            return tree_count;
        }

        if buffer[position.y * width + (position.x % width)] == Object::Tree {
            tree_count += 1;
        }
    }
}

fn main() {
    let input = read_to_string("./inputs/day03.txt").expect("file not found");

    let buffer: Vec<Object> = input
        .chars()
        .filter(|c| *c != '\n')
        .map(|c| if c == '#' { Object::Tree } else { Object::Snow })
        .collect();
    let width = input.lines().nth(0).unwrap().chars().count();
    let height = buffer.len() / width;

    println!("{}, {}, {}", width, height, buffer.len());

    // Task 1
    let direction = Vec2D::new(3, 1);
    // Result: 265
    println!("{}", check_slope(&buffer, width, height, &direction));

    // Task 2
    let directions = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| Vec2D::new(x.clone(), y.clone()))
        .collect::<Vec<_>>();

    // Result: 3154761400
    println!(
        "{:?}",
        directions
            .iter()
            .map(|d| check_slope(&buffer, width, height, &d))
            .fold(1, |acc, x| acc * x)
    );
}
