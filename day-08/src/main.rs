use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../data/input.txt");
    let (bottom_right, locations) = parse(input);

    let part1 = part1(bottom_right, &locations);

    println!("Part 1\t{}", part1);
    let part2 = part2(bottom_right, &locations);

    println!("Part 2\t{}", part2);
}

fn parse(input: &str) -> (Coord, HashMap<char, Vec<Coord>>) {
    let mut locations: HashMap<char, Vec<Coord>> = HashMap::new();

    input.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, value)| {
            // throw away the dots
            if value != '.' {
                let coord = Coord {
                    row: row as isize,
                    col: col as isize,
                };
                locations.entry(value).or_default().push(coord);
            }
        });
    });

    let bottom_right = Coord {
        row: input.lines().count() as isize,
        col: input.lines().next().unwrap().chars().count() as isize,
    };

    return (bottom_right, locations);
}

fn part1(bottom_right: Coord, locations: &HashMap<char, Vec<Coord>>) -> usize {
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in locations.values() {
        for c1 in coords {
            for c2 in coords {
                if c1 == c2 {
                    continue;
                }
                let distance = c1.sub(c2);

                let new = c1.add(&distance);

                if new.is_in_bounds(&bottom_right) {
                    antinodes.insert(new);
                }
            }
        }
    }
    return antinodes.len();
}


fn part2(bottom_right: Coord, locations: &HashMap<char, Vec<Coord>>) -> usize {
    let mut antinodes: HashSet<Coord> = HashSet::new();

    for coords in locations.values() {
        for c1 in coords {
            for c2 in coords {
                if c1 == c2 {
                    continue;
                }
                let distance = c1.sub(c2);

                // Antenna are antinodes
                let mut new = *c1;

                while new.is_in_bounds(&bottom_right) {
                    antinodes.insert(new);
                    // Skip forward after each insertion
                    new = new.add(&distance);
                }
            }
        }
    }
    return antinodes.len();
}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
struct Coord {
    row: isize,
    col: isize,
}

impl Coord {
    fn sub(&self, other: &Coord) -> Coord {
        return Coord {
            row: self.row - other.row,
            col: self.col - other.col,
        };
    }

    fn add(&self, other: &Coord) -> Coord {
        return Coord {
            row: self.row + other.row,
            col: self.col + other.col,
        };
    }

    fn is_in_bounds(&self, bottom_right: &Coord) -> bool {
        let is_in_bound_row = self.row >= 0 && self.row < bottom_right.row;
        let is_in_bound_col = self.col >= 0 && self.col < bottom_right.col;
        return is_in_bound_row && is_in_bound_col;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../data/test.txt");
        let (bottom_right, locations) = parse(input);
    
        let result = part1(bottom_right, &locations);
    
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../data/test.txt");
        let (bottom_right, locations) = parse(input);
    
        let result = part2(bottom_right, &locations);
    
        assert_eq!(result, 34);
    }
}