use std::ops::Rem;

fn main() {
    let input = include_str!("../data/input.txt");

    let part1 = part1(input);

    println!("Part 1:\t{}", part1);
}

fn part1(disk_map: &str) -> u64 {
    let current_layout = parse(disk_map);
    let defraged = defrag(current_layout);
    let checksum = checksum(defraged);

    return checksum;
}

fn parse(disk_map: &str) -> Vec<Option<u64>> {
    let mut layout = vec![];
    disk_map.chars().enumerate().for_each(|(ix,c)| {
        let count = c.to_digit(10).unwrap();
        let to_add = match ix.rem(2) {
            0 => Some((ix / 2) as u64),
            _ => None
        };
        for _ in 0..count {
            layout.push(to_add);
        }
    });
    return layout;
}

fn defrag(current_layout: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut new_layout = current_layout.clone();

    // Walk from the end forwards
    let mut loop_count = new_layout.len() - 1;
    while loop_count > 0 {
        let item = new_layout.get(loop_count).unwrap();
        
        // Already free space, leave it be
        if item.is_none() {
            loop_count -= 1;
            continue;
        }

        // Need to bring this forwards to the first free space
        // But not past the point we're looking at!
        let free = new_layout[0..loop_count].iter().position(|block| block.is_none());

        if let Some(free_ix) = free {
            new_layout.swap(free_ix, loop_count);
        }

        loop_count -= 1;
    }
    return new_layout;
}

fn checksum(layout: Vec<Option<u64>>) -> u64 {
    return layout.iter().enumerate().filter_map(|(ix,file)| {
        match file {
            Some(file) => Some(file*ix as u64),
            None => None
            }
    }).sum();
}

fn part2() -> usize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

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