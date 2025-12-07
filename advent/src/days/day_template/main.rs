use std::fs;


pub fn read_input(path: &str) -> Vec<String> {
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    contents.lines()
        .map(|line| line.to_string())
        .collect()
}

pub fn run_part_1(input: Vec<String>) -> usize {
    // TODO: Implement part 1 solution
    0
}

pub fn run_part_2(input: Vec<String>) -> usize {
    // TODO: Implement part 2 solution
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let result = read_input("src/days/day_template/input_files/test_input.txt");
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_run_part_1() {
        let input = read_input("src/days/day_template/input_files/test_input.txt");
        let result = run_part_1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_run_part_2() {
        let input = read_input("src/days/day_template/input_files/test_input.txt");
        let result = run_part_2(input);
        assert_eq!(result, 0);
    }
}
