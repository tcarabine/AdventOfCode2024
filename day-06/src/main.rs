use std::thread;

const VISITED: char = 'X';

fn main() {
    let input = include_str!("../data/input.txt");
    //let input = include_str!("../data/test.txt");

    let mut world: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let base_guard = find_guard(&world);

    let part_1 = part1(base_guard.clone(), &mut world);
    println!("{}", part_1);

    let part_2 = part2(base_guard.clone(), &mut world);
    //println!("{:?}", world);
    println!("{}", part_2);
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
struct Guard {
    location: (isize, isize),
    direction: Direction,
}

fn turn(direction: &Direction) -> Direction {
    return match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    };
}

fn get_movement(direction: &Direction) -> (isize, isize) {
    return match direction {
        Direction::Up => (0, -1),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
    };
}

fn walk(guard: &mut Guard, world: &mut Vec<Vec<char>>) {
    // Set current position to a Visited Char = let's use X
    let current = world
        .get_mut(guard.location.1 as usize)
        .unwrap()
        .get_mut(guard.location.0 as usize)
        .unwrap();
    *current = VISITED;

    // Get next position
    let movement = get_movement(&guard.direction);
    let next_loc = (guard.location.0 + movement.0, guard.location.1 + movement.1);

    let next_y = world.get(next_loc.1 as usize);
    let mut next = Option::None;

    if let Some(row) = next_y {
        next = row.get(next_loc.0 as usize);
    }

    // Check if I can move
    match next {
        Some('#') => guard.direction = turn(&guard.direction),
        _ => guard.location = next_loc,
    }
}

fn find_guard(world: &Vec<Vec<char>>) -> Guard {
    let g = world
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, c)| match c {
                    '^' => Some(Guard {
                        location: (x.try_into().unwrap(), y.try_into().unwrap()),
                        direction: Direction::Up,
                    }),
                    '>' => Some(Guard {
                        location: (x.try_into().unwrap(), y.try_into().unwrap()),
                        direction: Direction::Right,
                    }),
                    'v' => Some(Guard {
                        location: (x.try_into().unwrap(), y.try_into().unwrap()),
                        direction: Direction::Down,
                    }),
                    '<' => Some(Guard {
                        location: (x.try_into().unwrap(), y.try_into().unwrap()),
                        direction: Direction::Left,
                    }),
                    _ => None,
                })
                .filter(|val| val.is_some())
                .reduce(|_, val| val)
                .unwrap_or(None)
        })
        .filter(|val| val.is_some())
        .reduce(|_, val| val)
        .unwrap();
    return g.unwrap();
}

fn has_exited(guard: &Guard, bottom: isize, right: isize) -> bool {
    let top = 0;
    let left = 0;

    match guard.location {
        //left
        loc if loc.0 < left => return true,
        //right
        loc if loc.0 >= right => return true,
        //top
        loc if loc.1 < top => return true,
        //bottom
        loc if loc.1 >= bottom => return true,
        //else
        _ => return false,
    }
}

fn part1(mut guard: Guard, mut world: &mut Vec<Vec<char>>) -> usize {
    let bottom = world.len() as isize;

    let right = world.first().unwrap().len() as isize;

    while !has_exited(&guard, bottom, right) {
        walk(&mut guard, &mut world);
    }

    return world
        .iter()
        .map(|row| row.iter().filter(|c| c == &&VISITED).count())
        .reduce(|acc, count| acc + count)
        .unwrap();
}

fn walk_again(guard: &mut Guard, world: &mut Vec<Vec<char>>) -> bool {
    // Set current position to a show the movement
    let current = world
        .get_mut(guard.location.1 as usize)
        .unwrap()
        .get_mut(guard.location.0 as usize)
        .unwrap();

    match current {
        // I've been here 9 times. I'm stuck
        '9' => return true,
        '8' => *current = '9',
        '7' => *current = '8',
        '6' => *current = '7',
        '5' => *current = '6',
        '4' => *current = '5',
        '3' => *current = '4',
        '2' => *current = '3',
        '1' => *current = '2',
        _ => *current = '1',
    }

    // Get next position
    let movement = get_movement(&guard.direction);
    let next_loc = (guard.location.0 + movement.0, guard.location.1 + movement.1);

    let next_y = world.get(next_loc.1 as usize);
    let mut next = Option::None;

    if let Some(row) = next_y {
        next = row.get(next_loc.0 as usize);
    }
    // Check if I can move
    match next {
        Some('#') | Some('O') => guard.direction = turn(&guard.direction),
        _ => guard.location = next_loc,
    }
    return false;
}

fn generate_new_world(old_world: &&Vec<Vec<char>>, update: (usize, usize)) -> Vec<Vec<char>> {
    let mut new_world: Vec<Vec<char>> = old_world.iter().map(|row| row.clone()).collect();
    let to_change = new_world
        .get_mut(update.0)
        .unwrap()
        .get_mut(update.1)
        .unwrap();
    *to_change = 'O';
    return new_world;
}

fn test_for_loops(mut guard: Guard, mut world: &mut Vec<Vec<char>>) -> bool {
    let bottom = world.len() as isize;

    let right = world.first().unwrap().len() as isize;

    let mut testing = true;
    let mut loop_found = false;
    while testing {
        //println!("{:?}", world);
        loop_found = walk_again(&mut guard, &mut world);
        let escaped = has_exited(&guard, bottom, right);
        if loop_found {
            //println!("{:?}", world);
            testing = false
        } else if escaped {
            testing = false
        }
    }

    return loop_found;
}

fn part2(guard: Guard, world: &Vec<Vec<char>>) -> usize {
    world
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == VISITED)
                .map(|(x, _)| (y, x))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .filter(|loc| *loc != (guard.location.0 as usize, guard.location.1 as usize))
        .map(|update| {
            let mut test_world = generate_new_world(&world, update);
            let my_guard = guard.clone();
            return thread::spawn(move || test_for_loops(my_guard, &mut test_world));
        })
        .map(|t| t.join().unwrap())
        .filter(|result| *result)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let mut input = include_str!("../data/test.txt")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let guard = find_guard(&input);
        let result = part1(guard, &mut input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_input_part2() {
        let mut input = include_str!("../data/test.txt")
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let guard = find_guard(&input);
        let _ = part1(guard.clone(), &mut input);
        let result = part2(guard.clone(), &input);
        assert_eq!(result, 6);
    }
}
