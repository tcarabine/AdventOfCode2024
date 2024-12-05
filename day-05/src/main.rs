use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let input: Vec<&str> = include_str!("../data/input.txt").lines().collect();
    //let input: Vec<&str> = include_str!("../data/test.txt").lines().collect();
    let part_1 = part_1(input);

    println!("{}", part_1);
}

fn part_1(input: Vec<&str>) -> i32 {
    let rules = generate_rules(&input);
    let updates = generate_updates(&input);

    let valid: Vec<&Vec<i32>> = updates.iter().filter(|update| apply_rules(update, &rules)).collect();

    return valid.iter().map(|update| get_middle(update)).reduce(|acc,x| acc+x).unwrap();
}

fn part_2(input: Vec<&str>) -> i32 {
    let rules = generate_rules(&input);
    let updates = generate_updates(&input);

    let invalid: Vec<&Vec<i32>> = updates.iter().filter(|update| !apply_rules(update, &rules)).collect();

    
    return 0;
}

fn generate_rules(input: &Vec<&str>) -> HashMap<i32,Vec<i32>> {
    let mut rules = HashMap::new();

    input.iter().filter(|line| line.contains("|")).for_each(|rule| {
        let (k,v) = parse_rule(rule);
            rules.entry(k).and_modify(|val: &mut Vec<i32>| val.push(v)).or_insert(vec![v]);
    });
    return rules
}

fn parse_rule(rule: &str) -> (i32,i32) {
    let mut split = rule.split("|").map(|x| x.parse::<i32>().unwrap());
    let key =  split.next().unwrap();
    let value = split.next().unwrap();
    return (key,value)
}

fn generate_updates(input: &Vec<&str>) -> Vec<Vec<i32>> {
    let mut updates: Vec<Vec<i32>> = Vec::new();
    input.iter().filter(|line| line.contains(",")).for_each(|update| {
        updates.push(parse_update(update));
    });
    return updates
}

fn parse_update(update: &str) -> Vec<i32> {
    return update.split(",").map(|page| page.parse::<i32>().unwrap()).collect();
}

fn apply_rules(update: &Vec<i32>, rules: &HashMap<i32,Vec<i32>>) -> bool {
    return update.iter().enumerate().all(|(ix,value)| {
        match rules.get(value) {
            None => true,
            Some(afters) => apply_rule(ix, afters, update)
        }
    });
}

fn apply_rule(pos: usize, afters: &Vec<i32>, update: &Vec<i32>) -> bool {
    return update.iter().enumerate().filter(|(_,value)| afters.contains(value)).all(|(ix,_)| ix > pos);
}

fn get_middle(update: &Vec<i32>) -> i32 {
    let middle_pos = (update.len() -1) / 2;
    return *update.get(middle_pos).unwrap();
}

fn fix_update(update: &Vec<i32>, rules: &HashMap<i32,Vec<i32>>) -> Vec<i32> {
    return update.to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = include_str!("../data/test.txt").lines().collect();

        let result = part_1(input);

        assert_eq!(result, 143);
    }

    #[test]
    fn test_input_part2() {
        let input = include_str!("../data/test.txt").lines().collect();

        let result = part_2(input);

        assert_eq!(result, 123);
    }
}