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
        rows.iter().enumerate().for_each(|(y, row)| {
            let char = row.chars().nth(x + y);
            match char {
                Some(char) => diag.push(char),
                None => (),
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
        rows.iter().enumerate().skip(y).for_each(|(x, row)| {
            let char = row.chars().nth(x - y);

            match char {
                Some(char) => diag.push(char),
                None => (),
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
    let mut total: usize = 0;

    // Populate the search space
    search_space.extend(rows.iter().filter(|x| x.len() >= term.len()));
    search_space.extend(columns.iter().filter(|x| x.len() >= term.len()));

    // Top Left - Bottom Right Horizontally
    let tlbr_h: Vec<String> = horizontal_diags(&rows);
    // Top Left - Bottom Right Vertically
    let tlbr_v: Vec<String> = vertical_diags(&rows)
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect();
    search_space.extend(tlbr_h.iter().filter(|x| x.len() >= term.len()));
    search_space.extend(tlbr_v.iter().filter(|x| x.len() >= term.len()));

    let flipped: Vec<String> = rows.iter().rev().map(|x| x.to_string()).collect();
    // Bottom Left - Top Right Horizontally
    let bltr_h: Vec<String> = horizontal_diags(&flipped);
    // Top Left - Bottom Right Vertically
    let bltr_v: Vec<String> = vertical_diags(&flipped)
        .iter()
        .skip(1)
        .map(|x| x.to_string())
        .collect();
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
    return total;
}

fn explode_string(input: &str) -> Vec<String> {
    let result: Vec<String> = input.chars().map(|c| c.to_string()).collect();

    return result;
}

fn is_top_bottom_x(top: &[String], bottom: &[String]) -> bool {
    let tl = top.get(0).unwrap();
    let tr = top.get(2).unwrap();
    let bl = bottom.get(0).unwrap();
    let br = bottom.get(2).unwrap();

    if tl != "S" && tl != "M" {
        return false;
    }
    if tr != "S" && tr != "M" {
        return false;
    }
    if bl != "S" && bl != "M" {
        return false;
    }
    if br != "S" && br != "M" {
        return false;
    }

    if tl == tr && tl != bl && bl == br {
        //M-M
        //S-S

        //S-S
        //M-M
        return true;
    }

    if tl == bl && tl != tr && tr == br {
        //M-S
        //M-S

        //S-M
        //S-M
        return true;
    }
    return false;
}

fn part2(input: &Vec<&str>) -> usize {
    // Each row to row_max - 2
    // Window of 3 finding M*S or S*M
    // Each find, check start
    let rows = input.clone();
    let mut total = 0;
    let blocks_of_rows = rows.windows(3);

    for block in blocks_of_rows {
        let row_0 = explode_string(block[0]);
        let row_1 = explode_string(block[1]);
        let row_2 = explode_string(block[2]);

        let mut row_0_iter = row_0.windows(3);
        let mut row_1_iter = row_1.windows(3);
        let mut row_2_iter = row_2.windows(3);

        while let Some(r0) = row_0_iter.next() {
            let r1 = row_1_iter.next().unwrap();
            let r2 = row_2_iter.next().unwrap();

            if r1.get(1).unwrap() == "A" && is_top_bottom_x(r0, r2) {
                total += 1;
            }
        }
    }
    return total;
}

fn main() {
    let input = include_str!("../data/input.txt").lines().collect();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1\t{}", part_1);
    println!("Part 2\t{}", part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rowstocols_will_pivot_input() {
        let input = Vec::from(["ABCD".to_string(), "EFGH".to_string(), "IJKL".to_string()]);

        let result = rows_to_cols(&input);

        assert_eq!(result.get(0).unwrap(), "AEI");
        assert_eq!(result.get(1).unwrap(), "BFJ");
        assert_eq!(result.get(2).unwrap(), "CGK");
        assert_eq!(result.get(3).unwrap(), "DHL");
        assert_eq!(result.get(4), None);
    }

    #[test]
    fn top_left_bottom_right_horizontally() {
        let input = Vec::from(["ABCD".to_string(), "EFGH".to_string(), "IJKL".to_string()]);

        let result = horizontal_diags(&input);

        assert_eq!(result.get(0).unwrap(), "AFK");
        assert_eq!(result.get(1).unwrap(), "BGL");
        assert_eq!(result.get(2).unwrap(), "CH");
        assert_eq!(result.get(3).unwrap(), "D");
        assert_eq!(result.get(4), None);
    }

    #[test]
    fn top_left_bottom_right_vertically() {
        let input = Vec::from(["ABCD".to_string(), "EFGH".to_string(), "IJKL".to_string()]);

        let result = vertical_diags(&input);

        assert_eq!(result.get(0).unwrap(), "AFK");
        assert_eq!(result.get(1).unwrap(), "EJ");
        assert_eq!(result.get(2).unwrap(), "I");
        assert_eq!(result.get(3), None);
    }

    #[test]
    fn bottom_left_top_right_horizontally() {
        let mut input = Vec::from(["ABCD".to_string(), "EFGH".to_string(), "IJKL".to_string()]);
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
        let mut input = Vec::from(["ABCD".to_string(), "EFGH".to_string(), "IJKL".to_string()]);
        input.reverse();
        let result = vertical_diags(&input);

        assert_eq!(result.get(0).unwrap(), "IFC");
        assert_eq!(result.get(1).unwrap(), "EB");
        assert_eq!(result.get(2).unwrap(), "A");
        assert_eq!(result.get(3), None);
    }

    #[test]
    fn testing_for_x_shape() {
        let top: Vec<String> = vec!["S".to_string(), "-".to_string(), "S".to_string()];
        let bottom: Vec<String> = vec!["M".to_string(), "-".to_string(), "M".to_string()];

        let result = is_top_bottom_x(top.as_slice(), bottom.as_slice());
        assert_eq!(result, true);
    }

    #[test]
    fn part1_test_input() {
        let input = include_str!("../data/test.txt").lines().collect();
        let result = part1(&input);

        assert_eq!(result, 18);
    }

    #[test]
    fn part2_test_input() {
        let input = include_str!("../data/test.txt").lines().collect();
        let result = part2(&input);

        assert_eq!(result, 9);
    }
}
