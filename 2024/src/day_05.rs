use std::collections::{HashMap, HashSet};

pub fn run_day_4() {
    let input = std::fs::read_to_string("input/04.txt").unwrap();
    println!("Day 4 Part 1: {}", part_1(&input));
    println!("Day 4 Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> i32 {
    let (map, updates) = parse(input);
    updates
        .iter()
        .filter(|update| is_valid_page(update, &map))
        .map(|update| dbg!(update[dbg!(update).len() / 2]) as i32)
        .reduce(|acc, update| acc + update)
        .unwrap_or(0)
}

fn parse(input: &str) -> (Ruleset, Vec<Update>) {
    let mut sections = input.split(
        "

",
    );
    let (ordering_rules, update_pages) = (sections.next().unwrap(), sections.next().unwrap());
    let map = build_predecessor_map(ordering_rules);
    let updates = build_updates(update_pages);
    (map, updates)
}

type Ruleset = HashMap<usize, Vec<usize>>;
type Update = Vec<usize>;

fn is_valid_page(
    page: &Update,
    rules: &Ruleset,
) -> bool {
    let mut found: HashSet<usize> = HashSet::new();
    for page in page.iter() {
        found.insert(*page);
        for rule in rules.get(page).unwrap_or(&vec![]) {
            if found.contains(rule) {
                return false;
            }
        }
    }
    true
}

fn build_updates(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.split(',').map(|s| s.parse::<usize>().unwrap()).collect())
        .collect()
}

fn build_predecessor_map(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in input.lines() {
        let mut vals = line.split("|").map(|s| s.parse::<usize>().unwrap());
        let (before, after) = (vals.next().unwrap(), vals.next().unwrap());
        let after_reqs = map.entry(before).or_insert(Vec::new());
        after_reqs.push(after);
    }
    map
}

fn part_2(input: &str) -> i32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .map(|update| {
            if is_valid_page(&update, &rules) {
                update
            } else {
                reorder_update(update, &rules)
            }
        })
        .map(|update| dbg!(update[update.len() / 2]) as i32)
        .reduce(|acc, update| acc + update)
        .unwrap_or(0)
}

fn reorder_update(
    update: Update,
    rules: &Ruleset,
) -> Update {
    let mut numbers = update.into_iter().collect::<HashSet<_>>();
    let mut update = Vec::new();
    update
}

#[cfg(test)]
mod tests {
    use crate::day_05::part_1;
    use crate::day_05::part_2;

    #[test]
    fn test_part_1_sample_input() {
        let input = // Wrap to newline
"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!(143, part_1(input));
    }
    #[test]
    fn test_part_1() {
        let input = std::fs::read_to_string("inputs/05.txt").unwrap();
        assert_eq!(5108, part_1(&input));
    }
    #[test]
    fn test_part_2_sample_input() {
        let input = "";
        assert_eq!(143, part_2(input));
    }
    #[test]
    fn test_part_2() {
        let input = std::fs::read_to_string("inputs/05.txt").unwrap();
        assert_eq!(143, part_2(&input));
    }
}
