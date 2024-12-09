fn main() {
    let input = include_str!("../data/input.txt");
    //let input = include_str!("../data/test.txt");

    let part1 = part_1(input);
    println!("Part 1\t{}", part1);
    let part2 = part_2(input);
    println!("Part 2\t{}", part2);

}

fn parse_data(line: &str) -> (i64, Vec<i64>) {
    let mut split = line.split(':').into_iter();
    let key: i64 = split.next().unwrap().parse().unwrap();

    let value: Vec<i64> = split
        .next()
        .map(|str| {
            str.split_whitespace()
                .map(|val| val.parse::<i64>().unwrap())
        })
        .unwrap()
        .collect();
    return (key, value);
}

fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

fn mult(a: i64, b: i64) -> i64 {
    return a * b;
}

fn concat(a: i64, b: i64) -> i64 {
    let tmp = format!("{a}{b}");
    return tmp.parse().unwrap();
}

fn calibrates(result: &i64, values: &Vec<i64>) -> bool {
    // let mut operations:Vec<Vec<i64>> = vec![];
    let mut value_iter = values.iter();
    let mut current: Vec<i64> = vec![*value_iter.next().unwrap()];
    let mut last: Vec<i64>;
    while let Some(next_value) = value_iter.next() {
        last = current.clone();
        current = vec![];
        for last_value in last.iter() {
            current.push(sum(*last_value, *next_value));
            current.push(mult(*last_value, *next_value));
        }
    }

    return current.iter().any(|value| value == result);
}

fn part_1(input: &str) -> i64 {
    return input
        .lines()
        .map(|line| parse_data(line))
        .filter(|(key, value)| calibrates(key, value))
        .map(|(key, _)| key)
        .reduce(|acc, x| acc + x)
        .unwrap();
}

fn calibrates_concat(result: &i64, values: &Vec<i64>) -> bool {
    // let mut operations:Vec<Vec<i64>> = vec![];
    let mut value_iter = values.iter();
    let mut current: Vec<i64> = vec![*value_iter.next().unwrap()];
    let mut last: Vec<i64>;
    while let Some(next_value) = value_iter.next() {
        last = current.clone();
        current = vec![];
        for last_value in last.iter() {
            current.push(sum(*last_value, *next_value));
            current.push(mult(*last_value, *next_value));
            current.push(concat(*last_value, *next_value));
        }
    }

    return current.iter().any(|value| value == result);
}

fn part_2(input: &str) -> i64 {
    return input
        .lines()
        .map(|line| parse_data(line))
        .filter(|(key, value)| calibrates_concat(key, value))
        .map(|(key, _)| key)
        .reduce(|acc, x| acc + x)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let input = "190: 10 19";
        let expected_key = 190;
        let expected_value = vec![10, 19];

        let (key, value) = parse_data(input);
        assert_eq!(key, expected_key);
        assert_eq!(value, expected_value);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../data/test.txt");

        let result = part_1(input);

        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../data/test.txt");

        let result = part_2(input);

        assert_eq!(result, 11387);
    }
}
