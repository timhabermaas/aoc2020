use std::collections::HashSet;
use std::fmt;
use std::fs::read_to_string;

#[derive(Clone)]
struct World3d {
    actives: HashSet<(i64, i64, i64)>,
}

impl fmt::Display for World3d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bounding_box = Aabb::<(i64, i64, i64)>::from_coordinates(&self.actives);

        for z in bounding_box.min.2..=bounding_box.max.2 {
            write!(f, "z = {}\n", z)?;
            for y in bounding_box.min.1..=bounding_box.max.1 {
                for x in bounding_box.min.0..=bounding_box.max.0 {
                    if self.actives.contains(&(x, y, z)) {
                        write!(f, "#")?
                    } else {
                        write!(f, ".")?
                    }
                }
                write!(f, "\n")?
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
struct World4d {
    actives: HashSet<(i64, i64, i64, i64)>,
}
impl fmt::Display for World4d {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

trait World {
    type Coordinate;

    fn active_neighbours_count(&self, c: &Self::Coordinate) -> usize;

    fn points_to_check(&self) -> Vec<Self::Coordinate>;

    fn actives_count(&self) -> usize;

    fn cube_state(&self, c: &Self::Coordinate) -> State;

    fn insert_active(&mut self, c: Self::Coordinate);
    fn remove_active(&mut self, c: &Self::Coordinate);
}

impl World for World3d {
    type Coordinate = (i64, i64, i64);

    fn active_neighbours_count(&self, c: &Self::Coordinate) -> usize {
        let mut neighbours = vec![];

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if (x, y, z) != (0, 0, 0) {
                        neighbours.push((c.0 + x, c.1 + y, c.2 + z));
                    }
                }
            }
        }

        neighbours
            .iter()
            .filter(|c| self.actives.contains(c))
            .count()
    }

    fn cube_state(&self, c: &Self::Coordinate) -> State {
        if self.actives.contains(&c) {
            State::Active
        } else {
            State::Inactive
        }
    }

    fn insert_active(&mut self, c: Self::Coordinate) {
        self.actives.insert(c);
    }

    fn remove_active(&mut self, c: &Self::Coordinate) {
        self.actives.remove(c);
    }

    fn points_to_check(&self) -> Vec<Self::Coordinate> {
        let mut min = (i64::MAX, i64::MAX, i64::MAX);
        let mut max = (i64::MIN, i64::MIN, i64::MIN);

        for (x, y, z) in self.actives.clone() {
            if x < min.0 {
                min.0 = x;
            }
            if y < min.1 {
                min.1 = y;
            }
            if z < min.2 {
                min.2 = z;
            }
            if x > max.0 {
                max.0 = x;
            }
            if y > max.1 {
                max.1 = y;
            }
            if z > max.2 {
                max.2 = z;
            }
        }
        let aabb = Aabb { min, max };

        let mut result = Vec::new();
        for x in aabb.min.0 - 1..=aabb.max.0 + 1 {
            for y in aabb.min.1 - 1..=aabb.max.1 + 1 {
                for z in aabb.min.2 - 1..=aabb.max.2 + 1 {
                    result.push((x, y, z));
                }
            }
        }
        result
    }

    fn actives_count(&self) -> usize {
        self.actives.len()
    }
}

impl World for World4d {
    type Coordinate = (i64, i64, i64, i64);

    fn active_neighbours_count(&self, c: &Self::Coordinate) -> usize {
        let mut neighbours = vec![];

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in -1..=1 {
                        if (x, y, z, w) != (0, 0, 0, 0) {
                            neighbours.push((c.0 + x, c.1 + y, c.2 + z, c.3 + w));
                        }
                    }
                }
            }
        }

        neighbours
            .iter()
            .filter(|c| self.actives.contains(c))
            .count()
    }

    fn cube_state(&self, c: &Self::Coordinate) -> State {
        if self.actives.contains(&c) {
            State::Active
        } else {
            State::Inactive
        }
    }

    fn insert_active(&mut self, c: Self::Coordinate) {
        self.actives.insert(c);
    }

    fn remove_active(&mut self, c: &Self::Coordinate) {
        self.actives.remove(c);
    }

    fn points_to_check(&self) -> Vec<Self::Coordinate> {
        let mut min = (i64::MAX, i64::MAX, i64::MAX, i64::MAX);
        let mut max = (i64::MIN, i64::MIN, i64::MIN, i64::MIN);

        for (x, y, z, w) in self.actives.clone() {
            if x < min.0 {
                min.0 = x;
            }
            if y < min.1 {
                min.1 = y;
            }
            if z < min.2 {
                min.2 = z;
            }
            if w < min.3 {
                min.3 = w;
            }
            if x > max.0 {
                max.0 = x;
            }
            if y > max.1 {
                max.1 = y;
            }
            if z > max.2 {
                max.2 = z;
            }
            if w > max.3 {
                max.3 = w;
            }
        }
        let aabb = Aabb { min, max };

        let mut result = Vec::new();
        for x in aabb.min.0 - 1..=aabb.max.0 + 1 {
            for y in aabb.min.1 - 1..=aabb.max.1 + 1 {
                for z in aabb.min.2 - 1..=aabb.max.2 + 1 {
                    for w in aabb.min.3 - 1..=aabb.max.3 + 1 {
                        result.push((x, y, z, w));
                    }
                }
            }
        }
        result
    }

    fn actives_count(&self) -> usize {
        self.actives.len()
    }
}

// trait has associated type Coordinate?

#[derive(Debug, PartialEq, Clone)]
enum State {
    Active,
    Inactive,
}

#[derive(Debug)]
struct Aabb<C> {
    min: C,
    max: C,
}

impl Aabb<(i64, i64, i64)> {
    fn from_coordinates(coordinates: &HashSet<(i64, i64, i64)>) -> Self {
        let mut min = (i64::MAX, i64::MAX, i64::MAX);
        let mut max = (i64::MIN, i64::MIN, i64::MIN);

        for (x, y, z) in coordinates {
            if *x < min.0 {
                min.0 = *x;
            }
            if *y < min.1 {
                min.1 = *y;
            }
            if *z < min.2 {
                min.2 = *z;
            }
            if *x > max.0 {
                max.0 = *x;
            }
            if *y > max.1 {
                max.1 = *y;
            }
            if *z > max.2 {
                max.2 = *z;
            }
        }
        Self { min, max }
    }
}

fn parse_state(c: char) -> State {
    match c {
        '.' => State::Inactive,
        '#' => State::Active,
        _ => panic!("not a valid character: {}", c),
    }
}

fn solve<W: World + Clone + fmt::Display>(world: &mut W, max_iterations: usize) -> usize {
    for _i in 0..max_iterations {
        println!("{}", world);
        let old_world = world.clone();

        for c in world.points_to_check() {
            let active_neighbours = old_world.active_neighbours_count(&c);
            if old_world.cube_state(&c) == State::Active && !(2..=3).contains(&active_neighbours) {
                world.remove_active(&c);
            } else if old_world.cube_state(&c) == State::Inactive && active_neighbours == 3 {
                world.insert_active(c);
            }
        }
    }

    world.actives_count()
}

type Coordinate3d = <World3d as World>::Coordinate;

fn main() {
    let content = read_to_string("./inputs/day17.txt").expect("file not found");

    let world: HashSet<Coordinate3d> = content
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64, 0_i64), parse_state(c)))
        })
        .collect::<Vec<_>>()
        .iter()
        .filter(|(_c, v)| *v == State::Active)
        .map(|(c, _v)| c)
        .copied()
        .collect();

    // Part 1
    let mut w3 = World3d {
        actives: world.clone(),
    };
    // Result: 209
    println!("{}", solve(&mut w3, 6));

    // Part 2
    let mut w4 = World4d {
        actives: world.iter().map(|(x, y, z)| (*x, *y, *z, 0_i64)).collect(),
    };
    // Result: 1492
    println!("{}", solve(&mut w4, 6));
}
