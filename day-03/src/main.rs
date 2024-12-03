use regex::Regex;
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

fn find_matches_on_line(line: &str, re: &Regex) -> Vec<(u32, u32)> {
    return re
        .captures_iter(line)
        .map(|cap| (cap[1].parse().unwrap(), cap[2  ].parse().unwrap()))
        .collect();
}

fn process_commands(input: &str, command_re: &Regex, mul_re: &Regex) -> u32 {
    let mut process_mul = true;
    let matches: Vec<&str> = command_re.find_iter(input).map(|m| m.as_str()).collect();

    let mut acc: u32 = 0;
    for command in matches {
        match command {
            "do()" => process_mul = true,
            "don't()" => process_mul = false,
            _ => {
                if process_mul {
                    let cap = mul_re.captures(command).unwrap();
                    let result: u32 = cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
                    acc = acc + result;
                }
            }
        }
    }
    return acc;
}

fn main() {
    // let filename = "data/test.txt";
    //let filename = "data/test_2.txt";
    let filename = "data/input.txt";

    let data = load_data(filename);

    let part_1_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let part_1: u32 = data
        .iter()
        .map(|line| {
            find_matches_on_line(line, &part_1_re)
                .iter()
                .map(|(a, b)| a * b)
                .reduce(|acc, x| acc + x)
                .unwrap()
        })
        .reduce(|acc, x| acc + x)
        .unwrap();

    println!("Part 1:\t{part_1}");

    let part_2_re = Regex::new(r"(do\(\))|(mul\(\d{1,3},\d{1,3}\))|(don't\(\))").unwrap();

    let part_2 = process_commands(&(data.join("")), &part_2_re, &part_1_re);
    println!("Part 2: {part_2}");
}
