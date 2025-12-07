use std::fs;


pub fn read_input(path: &str) -> Vec<Vec<usize>> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    contents.lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10).map(|d| d as usize)).collect())
        .collect()
}

fn find_largest_2_digit_number(numbers: &Vec<usize>) -> usize {
    let mut max_number = 0;
    // println!("Checking array: {:?}", &numbers);
    for i in 0..numbers.len() - 1 {
        for j in i+1..numbers.len(){
            // println!("Checking subset: {:?}", &numbers[i..=j]);
            let two_digit_number = numbers[i] * 10 + numbers[j];
            if two_digit_number > max_number {
                max_number = two_digit_number;
            }
        }
        // println!("Current max number: {}", max_number);
    }
    max_number
}

pub fn run_part_1(input: Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    for numbers in input {
        // println!("Processing line: {:?}", numbers);
        let largest_2_digit_number = find_largest_2_digit_number(&numbers);
        sum += largest_2_digit_number;
        // println!("Largest 2-digit number: {}", largest_2_digit_number);
    }
    sum
}

pub fn run_part_2(input: Vec<Vec<usize>>) -> usize {
    input.len();
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let result = read_input("src/days/day3/input_files/test_input.txt");
        assert_eq!(result, vec![
            vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
            vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
            vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
            vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]
        ]);
    }

    #[test]
    fn test_ffind_largest_2_digit_number(){
        assert_eq!(find_largest_2_digit_number(&vec![9,8,7,6,5,4,3,2,1,1,1,1,1,1,1]), 98);
        assert_eq!(find_largest_2_digit_number(&vec![8,1,1,1,1,1,1,1,1,1,1,1,1,1,9]), 89);
        assert_eq!(find_largest_2_digit_number(&vec![2,3,4,2,3,4,2,3,4,2,3,4,2,7,8]), 78);
        assert_eq!(find_largest_2_digit_number(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]), 92);
    }

    #[test]
    fn test_run_part_1() {
        let input = read_input("src/days/day3/input_files/test_input.txt");
        let result = run_part_1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_run_part_2() {
        let input = read_input("src/days/day3/input_files/test_input.txt");
        let result = run_part_2(input);
        assert_eq!(result, 0);
    }
}
