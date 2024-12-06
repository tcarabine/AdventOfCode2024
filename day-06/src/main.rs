const VISITED: char = 'X';

fn main() {
    let input = include_str!("../data/input.txt");
    let world:Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let part_1 = part1(world);
    println!("{}",part_1);
}

enum Direction {
    Up,
    Right,
    Down,
    Left
}

struct Guard {
    location: (isize,isize),
    direction: Direction
}

fn turn(direction: &Direction) -> Direction {
    return match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn get_movement(direction: &Direction) -> (isize,isize) {
    return match direction {
        Direction::Up => (0,-1),
        Direction::Right => (1,0),
        Direction::Down => (0,1),
        Direction::Left => (-1,0),
    }
}

fn walk(guard: &mut Guard, world: &mut Vec<Vec<char>>) {
    // Set current position to a Visited Char = let's use X
    let current = world.get_mut(guard.location.1 as usize).unwrap().get_mut(guard.location.0 as usize).unwrap();
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
    let g= world
        .iter()
        .enumerate()
        .map(|(y,line)| {
            line
                .iter()
                .enumerate()
                .map(move |(x,c)| {
                    match c {
                        '^' => Some(Guard {
                            location: (x.try_into().unwrap(),y.try_into().unwrap()),
                            direction: Direction::Up
                        }),
                        '>' => Some(Guard {
                            location: (x.try_into().unwrap(),y.try_into().unwrap()),
                            direction: Direction::Right
                        }),
                        'v' => Some(Guard {
                            location: (x.try_into().unwrap(),y.try_into().unwrap()),
                            direction: Direction::Down
                        }),
                        '<' => Some(Guard {
                            location: (x.try_into().unwrap(),y.try_into().unwrap()),
                            direction: Direction::Left
                        }),
                        _ => None
                    }
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
        loc if loc.1 < top  => return true,
        //bottom
        loc if loc.1 >= bottom => return true,
        //else
        _ => return false
    }
}

fn part1(mut world:Vec<Vec<char>>) -> usize {
    let bottom = world.len() as isize;

    let right = world.first().unwrap().len() as isize;

    let mut guard = find_guard(&world);

    while !has_exited(&guard, bottom, right) {
        walk(&mut guard, &mut world);
    }

    return world
        .iter()
        .map(|row| {
            row
                .iter()
                .filter(|c| c == &&VISITED)
                .count()
            })
        .reduce(|acc,count| acc + count)
        .unwrap();
}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_input_part1() {
        let input = include_str!("../data/test.txt").lines().map(|line| line.chars().collect()).collect();
        println!("starting");
        let result = part1(input);
        assert_eq!(result,41);
    }
}