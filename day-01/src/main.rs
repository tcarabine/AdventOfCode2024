use std::fs::File;
use std::io::{BufRead, BufReader};
use std::thread;

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

fn main() {
    // let test_filename = "data/test.txt";
    // let data = load_data(test_filename);

    let filename = "data/input.txt";
    let data = load_data(filename);

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = (Vec::new(), Vec::new());

    for line in data.into_iter() {
        let split: Vec<i32> = line
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        left.push(split[0]);
        right.push(split[1]);
    }

    //Unstable refers to equivelence sorting. not actual stability
    let left_thread: thread::JoinHandle<Vec<i32>> = thread::spawn(move || {
        left.sort_unstable();
        return left;
    });
    let right_thread: thread::JoinHandle<Vec<i32>> = thread::spawn(move || {
        right.sort_unstable();
        return right;
    });

    left = left_thread.join().unwrap();
    right = right_thread.join().unwrap();

    let part1 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .reduce(|acc, d| acc + d)
        .unwrap();
    println!("Part 1: {part1}");

    // Part 2

    let part2 = left
        .iter()
        .map(|value| value * (right.iter().filter(|x| *x == value).count() as i32))
        .reduce(|acc, products| acc + products)
        .unwrap();

    println!("Part 2: {part2}");
}
