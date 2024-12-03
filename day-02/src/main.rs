use std::fs::File;
use std::io::{BufRead, BufReader};

fn load_data(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Cannot open File");

    let reader = BufReader::new(file);
    let data = reader.lines();
    let data: Vec<String> = data
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap())
        .filter(|line| !line.is_empty())
        .collect();
    return data;
}

fn less(left: &i32, right: &i32) -> bool {
    left.lt(right)
}

fn more(left: &i32, right: &i32) -> bool {
    left.gt(right)
}

fn up_or_down(left: &i32, right: &i32) -> impl Fn(&i32, &i32) -> bool {
    if left.gt(right) {
        return more as for<'a, 'b> fn(&'a i32, &'b i32) -> bool;
    } else {
        return less as for<'a, 'b> fn(&'a i32, &'b i32) -> bool;
    }
}

fn is_stable(left: &i32, right: &i32, comp: impl Fn(&i32, &i32) -> bool) -> bool {
    let following = comp(left, right);

    if following {
        let diff = left.abs_diff(*right);
        return diff.ge(&1) && diff.le(&3);
    }
    return false;
}

fn is_safe(readings: &Vec<i32>) -> bool {
    let comp = up_or_down(&readings[0], &readings[1]);
    return readings
        .windows(2)
        .all(|pair| is_stable(&pair[0], &pair[1], &comp));
}

fn can_be_made_safe(readings: &Vec<i32>) -> bool {
    for (remove, _) in readings.iter().enumerate() {
        let mut amended = readings.clone().clone();
        amended.remove(remove);
        if is_safe(&amended) {
            return true;
        }
    }
    return false;
}

fn main() {
    //let filename = "data/test.txt";
    let filename = "data/input.txt";
    let data = load_data(filename);

    let numbers: Vec<Vec<i32>> = data
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let part1 = numbers
        .iter()
        .filter(|readings| is_safe(*readings))
        .collect::<Vec<&Vec<i32>>>()
        .len();
    println!("Part 1: {part1}");

    // Part 2

    let part2 = numbers
        .iter()
        .filter(|readings| is_safe(*readings) || can_be_made_safe(*readings))
        .collect::<Vec<&Vec<i32>>>()
        .len();

    println!("Part 2: {part2}");
}
