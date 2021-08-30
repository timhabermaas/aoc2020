use std::boxed::Box;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
enum Rule {
    Or(Box<Rule>, Box<Rule>),
    List(Vec<usize>),
    Char(char),
    /// Corresponds to the regex a+b.
    Multiple(usize, usize),
}

type Rules = HashMap<usize, Rule>;

fn parse_number_list(list: &str) -> Vec<usize> {
    list.split(' ')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn parse_rule(rule: &str) -> Rule {
    if rule.chars().nth(0) == Some('"') {
        Rule::Char(rule.chars().nth(1).unwrap())
    } else if rule.contains('|') {
        let mut rules = rule.split('|').map(str::trim);
        Rule::Or(
            Box::new(Rule::List(parse_number_list(rules.next().unwrap()))),
            Box::new(Rule::List(parse_number_list(rules.next().unwrap()))),
        )
    } else {
        Rule::List(parse_number_list(rule))
    }
}

fn parse_rule_line(rule_line: &str) -> (usize, Rule) {
    let mut parts = rule_line.split(':');
    let number: usize = parts.next().unwrap().parse().unwrap();
    let rule = parse_rule(&parts.next().unwrap().trim());

    (number, rule)
}

fn parse_rules(rules: &str) -> Rules {
    rules.lines().map(parse_rule_line).collect()
}

fn parse_messages(messages: &str) -> Vec<&str> {
    messages.lines().collect()
}
// TODO: Each match should return a substring since a match consumes some part of the string.

fn matches_rule<'a>(rule: &Rule, rules: &Rules, message: &'a str) -> Option<&'a str> {
    match rule {
        Rule::Char(c) => {
            if message.is_empty() {
                return None;
            }
            let (left, right) = message.split_at(1);
            if left == c.to_string() {
                Some(right)
            } else {
                None
            }
        }
        Rule::Or(left, right) => {
            matches_rule(left, rules, message).or(matches_rule(right, rules, message))
        }
        Rule::List(list) => {
            let mut rest = message;
            for rule_index in list {
                rest = matches_rule(rules.get(rule_index)?, rules, rest)?;
            }
            Some(rest)
        }
        Rule::Multiple(min_one_offset, delimiter) => {
            //let first = matches_rule(rules.get(min_one_offset)?, rules, message)?;
            let mut rest = matches_rule(rules.get(min_one_offset)?, rules, message)?;
            loop {
                let del = matches_rule(rules.get(delimiter)?, rules, rest);
                if del.is_some() {
                    return del;
                }

                rest = matches_rule(rules.get(min_one_offset)?, rules, rest)?;
                //matches_rule(rules.get(min_one_offset)?, rules, rest);
            }
        }
    }
}

fn get_terminal_characters(rules: &Rules, rule: &Rule) -> Vec<char> {
    //let rule = rules.get(&rule).unwrap();

    match rule {
        Rule::Char(c) => vec![*c],
        Rule::List(list) => list
            .iter()
            .flat_map(|r| get_terminal_characters(rules, rules.get(r).unwrap()))
            .collect(),
        Rule::Multiple(a, b) => {
            let mut result = get_terminal_characters(rules, rules.get(a).unwrap());
            let mut second = get_terminal_characters(rules, rules.get(b).unwrap());
            result.append(&mut second);
            result
        }
        Rule::Or(r1, r2) => {
            let mut result = get_terminal_characters(rules, r1);
            let mut second = get_terminal_characters(rules, r2);
            result.append(&mut second);
            result
        }
    }
}

fn matches_rule_0(rules: &Rules, message: &str) -> bool {
    let rule_0 = rules.get(&0).expect("rule 0 should exist");
    matches_rule(&rule_0, &rules, &message)
        .filter(|str| str.is_empty())
        .is_some()
}

fn main() {
    let content = read_to_string("./inputs/day19.txt").expect("file not found");

    let mut parts = content.split("\n\n");

    let rules = parts.next().unwrap();
    let messages = parts.next().unwrap();

    let mut rules = parse_rules(&rules);
    let messages = parse_messages(&messages);

    for m in messages.clone() {
        println!("{}: {}", m, matches_rule_0(&rules, m));
    }
    // Result: 162
    println!(
        "{}",
        messages
            .iter()
            .filter(|m| matches_rule_0(&rules, m))
            .count()
    );

    // Try to match as many 42 as possible?
    /*
    rules.insert(
        8,
        Rule::Or(
            Box::new(Rule::List(vec![42])),
            Box::new(Rule::List(vec![42, 8])),
        ),
    );*/
    //rules.insert(8, Rule::Multiple(42, 8));
    //rules.insert(11, Rule::Multiple(42, 31));
    /*
    rules.insert(
        11,
        Rule::Or(
            Box::new(Rule::List(vec![42, 31])),
            Box::new(Rule::List(vec![42, 11, 31])),
        ),
    );*/
    /*
    println!(
        "{:?}",
        get_terminal_characters(&rules, rules.get(&42).unwrap())
    );
    println!(
        "{:?}",
        get_terminal_characters(&rules, rules.get(&31).unwrap())
    );*/
    println!(
        "{}",
        messages
            .iter()
            .filter(|m| matches_rule_0(&rules, m))
            .count()
    );
    //println!("{:?}", messages);
}
