use std::ops::Rem;

fn main() {
    println!("Hello, world!");
}

fn part1(disk_map: &str) -> usize {
    let mut current_layout: Vec<char> = parse(disk_map);
    let new = defrag(current_layout);


    return 0;
}

fn parse(disk_map: &str) -> Vec<char> {
    let mut layout = vec![];
    disk_map.chars().enumerate().for_each(|(ix,c)| {
        let count = c.to_digit(10).unwrap();
        let to_add = match ix.rem(2) {
            0 => char::from_digit((ix / 2) as u32, 10).unwrap(),
            _ => '.'
        };
        for _ in 0..count {
            layout.push(to_add);
        }
    });
    return layout;
}

fn defrag(current_layout: Vec<char>) -> Vec<char> {
    let mut new_layout = current_layout.clone();
    let free_space = current_layout.iter().enumerate().filter(|(_,c)| **c == '.').map(|(ix,_)| ix);
    for free in free_space {
        
        if free > new_layout.len() {
            continue;
        }
        // Need to check if about to swap in a . and if so, skip over it to get a char
        new_layout.swap_remove(free);
    }
    if new_layout.contains(&'.') {
        return defrag(new_layout);
    }
    return new_layout;
}

fn part2() -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_parse() {
        let input = "12345";
        let expected = "0..111....22222";

        let result = parse(input).iter().map(|c| c.to_string()).reduce(|acc,c| format!("{acc}{c}")).unwrap().to_owned();

        assert_eq!(result,expected);
    }

    #[test]
    fn test_bigger_parse() {
        let input = "2333133121414131402";
        let expected = "00...111...2...333.44.5555.6666.777.888899";

        let result = parse(input).iter().map(|c| c.to_string()).reduce(|acc,c| format!("{acc}{c}")).unwrap().to_owned();

        assert_eq!(result,expected);
    }
    #[test]
    fn test_defrag() {
        let input = "2333133121414131402";
        let expected = "0099811188827773336446555566";

        let parsed = parse(input);
        let result = defrag(parsed).iter().map(|c| c.to_string()).reduce(|acc,c| format!("{acc}{c}")).unwrap().to_owned();

        assert_eq!(result,expected);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../data/test.txt");

        let result = part1(input);

        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../data/test.txt");

        let result = part2();

        assert_eq!(result, 0);
    }
}