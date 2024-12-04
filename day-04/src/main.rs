fn count_word(haystack: &String, needle: &String) -> usize {
    let count = haystack.matches(needle).collect::<Vec<&str>>().len();
    return count;
}

fn rows_to_cols(rows: &Vec<String>) -> Vec<String> {
    let col_count = rows.get(0).unwrap().chars().count();

    let mut cols: Vec<String> = Vec::new();

    for ix in 0..col_count {
        let mut col = String::new();
        rows.iter().for_each(|row| {
            let char = row.chars().nth(ix).unwrap();
            col.push(char);
        });
        cols.push(col);
    }

    return cols;
}

fn horizontal_diags(rows: &Vec<String>) -> Vec<String> {
    let mut diags: Vec<String> = Vec::new();

    let x_max = rows.get(0).unwrap().chars().count();

    for x in 0..x_max {
        let mut diag = String::new();
        rows.iter().enumerate().for_each(|(y,row)| {
            let char = row.chars().nth(x + y);
            match char {
                Some(char) => diag.push(char),
                None => ()
            }
        });
        diags.push(diag);
    }
    return diags;
}

fn vertical_diags(rows: &Vec<String>) -> Vec<String> {
    let mut diags: Vec<String> = Vec::new();

    let y_max = rows.len();

    for y in 0..y_max {
        let mut diag = String::new();
        rows.iter().enumerate().skip(y).for_each(|(x,row)| {
            let char = row.chars().nth(x - y);

            match char {
                Some(char) => diag.push(char),
                None => ()
            }
        });
        diags.push(diag);
    }
    
    return diags;
}

fn part1(input: &Vec<&str>) -> usize {
    let rows: Vec<String> = input.iter().map(|line| line.to_string()).collect();
    let columns: Vec<String> = rows_to_cols(&rows);

    let term: String = "XMAS".to_string();
    let mert: String = term.chars().rev().collect::<String>();

    let mut search_space: Vec<&String> = Vec::new();
    let mut total:usize = 0;

    // Populate the search space
    search_space.extend(rows.iter().filter(|x| x.len() >= term.len()));
    search_space.extend(columns.iter().filter(|x| x.len() >= term.len()));  


    // Top Left - Bottom Right Horizontally
    let tlbr_h: Vec<String> = horizontal_diags(&rows);
    // Top Left - Bottom Right Vertically
    let tlbr_v: Vec<String> = vertical_diags(&rows).iter().skip(1).map(|x| x.to_string()).collect();
    search_space.extend(tlbr_h.iter().filter(|x| x.len() >= term.len()));
    search_space.extend(tlbr_v.iter().filter(|x| x.len() >= term.len()));  
    
    let flipped: Vec<String> = rows.iter().rev().map(|x| x.to_string()).collect();
    // Bottom Left - Top Right Horizontally
    let bltr_h: Vec<String> = horizontal_diags(&flipped);
    // Top Left - Bottom Right Vertically
    let bltr_v: Vec<String> = vertical_diags(&flipped).iter().skip(1).map(|x| x.to_string()).collect();
    search_space.extend(bltr_h.iter().filter(|x| x.len() >= term.len()));
    search_space.extend(bltr_v.iter().filter(|x| x.len() >= term.len()));  
    

    // Need to count all
    total += search_space
        .iter()
        .map(|line| count_word(line, &term))
        .reduce(|acc, val| acc + val)
        .unwrap_or(0);
    // And all of them backwards
    total += search_space
        .iter()
        .map(|line| count_word(line, &mert))
        .reduce(|acc, val| acc + val)
        .unwrap_or(0);
    return total
}

fn main() {
    let input = include_str!("../data/input.txt").lines().collect();

    let part_1 = part1(&input);

    println!("Part 1\t{}",part_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowstocols_will_pivot_input() {
        let input = Vec::from([
            "ABCD".to_string(), 
            "EFGH".to_string(), 
            "IJKL".to_string()
            ]);

        let result = rows_to_cols(&input);

        assert_eq!(result.get(0).unwrap(), "AEI");
        assert_eq!(result.get(1).unwrap(), "BFJ");
        assert_eq!(result.get(2).unwrap(), "CGK");
        assert_eq!(result.get(3).unwrap(), "DHL");
        assert_eq!(result.get(4), None);
    }

    #[test]
    fn top_left_bottom_right_horizontally() {
        let input = Vec::from([
            "ABCD".to_string(), 
            "EFGH".to_string(), 
            "IJKL".to_string()
            ]);

        let result = horizontal_diags(&input);

        assert_eq!(result.get(0).unwrap(), "AFK");
        assert_eq!(result.get(1).unwrap(), "BGL");
        assert_eq!(result.get(2).unwrap(), "CH");
        assert_eq!(result.get(3).unwrap(), "D");
        assert_eq!(result.get(4), None);
    }

    #[test]
    fn top_left_bottom_right_vertically() {
        let input = Vec::from([
            "ABCD".to_string(), 
            "EFGH".to_string(), 
            "IJKL".to_string()
            ]);
        
        let result = vertical_diags(&input);

        assert_eq!(result.get(0).unwrap(), "AFK");
        assert_eq!(result.get(1).unwrap(), "EJ");
        assert_eq!(result.get(2).unwrap(), "I");
        assert_eq!(result.get(3), None);
    }

    #[test]
    fn bottom_left_top_right_horizontally() {
        let mut input = Vec::from([
            "ABCD".to_string(), 
            "EFGH".to_string(), 
            "IJKL".to_string()
            ]);
        input.reverse();
        let result = horizontal_diags(&input);

        assert_eq!(result.get(0).unwrap(), "IFC");
        assert_eq!(result.get(1).unwrap(), "JGD");
        assert_eq!(result.get(2).unwrap(), "KH");
        assert_eq!(result.get(3).unwrap(), "L");
        assert_eq!(result.get(4), None);
    }

    #[test]
    fn bottom_left_top_right_vertically() {
        let mut input = Vec::from([
            "ABCD".to_string(), 
            "EFGH".to_string(), 
            "IJKL".to_string()
            ]);
        input.reverse();
        let result = vertical_diags(&input);

        assert_eq!(result.get(0).unwrap(), "IFC");
        assert_eq!(result.get(1).unwrap(), "EB");
        assert_eq!(result.get(2).unwrap(), "A");
        assert_eq!(result.get(3), None);
    }

    #[test]
    fn part1_test_input() {
        let input = include_str!("../data/test.txt").lines().collect();
        let result = part1(&input);

        assert_eq!(result,18);
    }
}
