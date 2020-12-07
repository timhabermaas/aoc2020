use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn parse_line(line: &str) -> (&str, Vec<(usize, &str)>) {
    let mut splits = line.split(" bags contain ");
    let bag_color = splits.next().unwrap();
    let rest = splits.next().unwrap();
    if rest == "no other bags." {
        return (bag_color, vec![]);
    }
    let contained_bags_iter = rest.split(", ").map(|c| {
        let index = c.find(" ").unwrap();
        let bag_count = c[0..index].parse().unwrap();
        let next_index = c[index..].find("bag").unwrap();
        let color = &c[index + 1..next_index];

        (bag_count, color)
    });
    (bag_color, contained_bags_iter.collect())
}
// First find all bags which directly contain a shiny gold bag. Then find all bags which contain
// the bags currently in the solution set. Iterate until the set doesn't get larger anymore.
// Fixpoint iteration.
fn find_contains(containers: &mut HashSet<String>, map: &HashMap<&str, Vec<&str>>) -> bool {
    let mut container_grew = false;
    for container in containers.clone().iter() {
        if let Some(cs) = map.get(container.as_str()) {
            for c in cs {
                // TODO: Avoid allocation.
                container_grew = container_grew || containers.insert(c.to_string());
            }
        }
    }
    container_grew
}

fn main() {
    let content = read_to_string("./inputs/day07.txt").expect("file not found");

    // map describes in which bags a color can be contained.
    let mut some_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in content.lines() {
        let (containing_bag, inner_bags) = parse_line(line);
        for (_, inner_bag) in inner_bags {
            let list = some_map.entry(inner_bag).or_insert(vec![]);
            list.push(containing_bag);
        }
    }

    let mut containers = HashSet::new();
    containers.insert("shiny gold".to_string());

    while find_contains(&mut containers, &some_map) == true {}

    // Result: 257
    // Subtracting 1 because "shiny gold" can't contain itself.
    println!("{:?}", containers.len() - 1);

    // Task 2:
    let mut containing_bags: HashMap<&str, Vec<(usize, &str)>> = HashMap::new();
    for line in content.lines() {
        let (containing_bag, inner_bags) = parse_line(line);
        containing_bags.insert(containing_bag, inner_bags);
    }

    // Result: 1038
    let result = count_bags("shiny gold", &containing_bags);
    println!("{}", result);
}

fn count_bags(bag_color: &str, containing_bags: &HashMap<&str, Vec<(usize, &str)>>) -> usize {
    match containing_bags.get(bag_color) {
        Some(bags) => bags
            .iter()
            .map(|(size, color)| size + size * count_bags(color, containing_bags))
            .sum(),
        None => 0,
    }
}
