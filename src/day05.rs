#[derive(Debug)]
struct Rule {
    before: u32,
    after: u32,
}

fn read(fname: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let content = std::fs::read_to_string(fname).unwrap();
    let mut sections = content.split("\n\n");

    // Parse rules
    let rules = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            Rule {
                before: parts.next().unwrap().parse().unwrap(),
                after: parts.next().unwrap().parse().unwrap(),
            }
        })
        .collect();

    // Parse updates
    let updates = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_valid_update(rules: &[Rule], update: &[u32]) -> bool {
    // Create a position map for this update
    let mut positions = std::collections::HashMap::new();
    for (pos, &num) in update.iter().enumerate() {
        positions.insert(num, pos);
    }

    // Check each rule
    for rule in rules {
        if let (Some(&pos1), Some(&pos2)) =
            (positions.get(&rule.before), positions.get(&rule.after))
        {
            if pos1 > pos2 {
                return false;
            }
        }
    }
    true
}

fn sort_invalid_update(rules: &[Rule], update: &[u32]) -> Vec<u32> {
    // Create subgraph for this update's numbers
    let mut in_degree: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
    let mut local_graph: std::collections::HashMap<u32, Vec<u32>> =
        std::collections::HashMap::new();

    // Initialize all numbers in this update
    for &num in update {
        in_degree.insert(num, 0);
        local_graph.insert(num, Vec::new());
    }

    // Add edges from rules that apply to this update
    for rule in rules {
        if update.contains(&rule.before) && update.contains(&rule.after) {
            local_graph.get_mut(&rule.before).unwrap().push(rule.after);
            *in_degree.get_mut(&rule.after).unwrap() += 1;
        }
    }

    // Topological sort
    let mut sorted = Vec::new();
    let mut queue: std::collections::VecDeque<u32> = in_degree
        .iter()
        .filter(|(_, &count)| count == 0)
        .map(|(&num, _)| num)
        .collect();

    while let Some(num) = queue.pop_front() {
        sorted.push(num);
        if let Some(neighbors) = local_graph.get(&num) {
            for &next in neighbors {
                *in_degree.get_mut(&next).unwrap() -= 1;
                if in_degree[&next] == 0 {
                    queue.push_back(next);
                }
            }
        }
    }

    sorted
}

pub fn part1(fname: &str) -> i32 {
    let (rules, updates) = read(fname);
    updates
        .iter()
        .filter(|update| is_valid_update(&rules, update))
        .map(|update| update[update.len() / 2] as i32)
        .sum()
}

pub fn part2(fname: &str) -> i32 {
    let (rules, updates) = read(fname);
    updates
        .iter()
        .filter(|update| !is_valid_update(&rules, update))
        .map(|update| {
            let sorted = sort_invalid_update(&rules, update);
            sorted[sorted.len() / 2] as i32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/05_sample.txt"), 143);
        assert_eq!(part1("data/05.txt"), 7074);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/05_sample.txt"), 123);
        assert_eq!(part2("data/05.txt"), 4828);
    }
}
