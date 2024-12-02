use std::fs::File;
use std::io::{BufRead, BufReader};


fn load_data(filename: &str) -> Vec<String>{
  let file = File::open(filename).expect("Cannot open File");

  let reader = BufReader::new(file);
  let data = reader.lines();
  let data: Vec<String> = data
    .filter(|result| result.is_ok())
    .map(|result| result.unwrap())
    .filter(|line| !line.is_empty())
    .collect();
  return data
}

fn less(left: &i32, right: &i32) -> bool {
  left.lt(right)
}

fn more(left: &i32, right: &i32) -> bool {
  left.gt(right)
}

fn up_or_down(left: &i32, right: &i32) -> impl Fn(&i32,&i32) -> bool {
  if left.gt(right)
  {
    return more as for<'a, 'b> fn(&'a i32, &'b i32) -> bool
  }
  else 
  {
    return less as for<'a, 'b> fn(&'a i32, &'b i32) -> bool
  }
}

fn is_stable(left: &i32, right: &i32, comp: impl Fn(&i32,&i32) -> bool) -> bool {
  let following = comp(left,right);

  if following {
    let diff = left.abs_diff(*right);
    return diff.ge(&1) && diff.le(&3)
  }
  else {
      false
  }
}

fn main() {
    //let test_filename = "data/test.txt";
    //let data = load_data(test_filename);

    let filename = "data/input.txt";
    let data = load_data(filename);
    
    let mut numbers = Vec::new();

    for line in data.iter() {
      let split: Vec<i32> = line
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
      numbers.push(split);
    }

    let filtered_lines: Vec<&Vec<i32>> = numbers.iter()
      .filter(|readings| {
        let first = readings.get(0).unwrap();
        let second = readings.get(1).unwrap();

        if first == second { return false }
        let cmp = up_or_down(first,second);
        
        let slice: &[i32] = &readings;
        let pairs: Vec<(&i32,&i32)> = slice
          .windows(2)
          .map(|s| (s.get(0).unwrap(),s.get(1).unwrap()))
          .filter(|(a,b)| is_stable(a,b,&cmp))
          .collect();
        
        return pairs.len() == slice.windows(2).len()
      })
      .collect();

    let part1 = filtered_lines.len() as i32;
    println!("Part 1: {part1}");

    // Part 2

    let part2 = 0;
    
    println!("Part 2: {part2}");

}
