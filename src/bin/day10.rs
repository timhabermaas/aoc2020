use std::fs::read_to_string;

fn main() {
    let content = read_to_string("./inputs/day10.txt").expect("file not found");

    let mut adapters: Vec<u64> = content
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

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
    println!("{}", ones * threes);

    // Second task: It kinda looks like a graph problem. With node 0 we have N edges to get to 1,
    // 2 and 3. To count the number of arrangements we have to count the number of paths through
    // that graph.
    // BFS and fold over outgoing edges by multiplying them?
    //
    // We might be able to define the graph in reverse order (starting with goal) and recursively
    // going back.
    // Topological sort? What's the data structure used to express the result of a topological
    // sort?
    // Our input list is naturally topologically sorted as long as the values are sorted.
    //
    // Combine topological sort + recursion? If we have a list of nodes we should be able to...???
}
