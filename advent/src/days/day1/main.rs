use std::fs;


pub fn read_input(path: &str) -> Vec<(char, usize)>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    contents.lines()
        .map(|line| {
            let direction = line.chars().next().unwrap();
            let number = line[1..].parse::<usize>().unwrap();
            (direction, number)
        })
        .collect()
}


pub fn rotate(direction: &char, current_position: usize, rotate_by: usize) -> usize {
    match direction {
        'R' => (current_position + rotate_by) % 100,
        'L' => (current_position as i32 - rotate_by as i32).rem_euclid(100).abs() as usize,
        _ => current_position,
    }
}

pub fn run_part_1(input: Vec<(char, usize)>) -> usize {
    let mut final_position = 50;
    let mut num_of_0_pos: usize = 0;
    for (direction, number) in input{
        // let last_position = final_position;
        // println!("Rotating from position: {}, direction: {}, by: {}", final_position, direction, number);
        final_position = rotate(&direction, final_position, number);
        // println!("Rotation complete! Current position: {}, moved to: {}, by {}; result = {}", last_position, direction, number, final_position);
        if final_position == 0 {
            num_of_0_pos += 1;
        }
    }
    num_of_0_pos
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input() {
        let result = read_input("src/days/day1/input_files/test_input.txt");
        assert_eq!(result, 
            vec![
                ('L', 68),
                ('L', 30),
                ('R', 48),
                ('L', 5),
                ('R', 60),
                ('L', 55),
                ('L', 1),
                ('L', 99),
                ('R', 14),
                ('L', 82)
            ]
        );
    }
    #[test]
    fn test_rotate() {
        assert_eq!(rotate(&'R', 0, 10), 10);
        assert_eq!(rotate(&'L', 10, 5), 5);
        assert_eq!(rotate(&'R', 95, 10), 5);
        assert_eq!(rotate(&'L', 5, 10), 95);
        assert_eq!(rotate(&'L', 5, 101), 4);
    }

    #[test]
    fn test_run_part_1() {
        let input = vec![
            ('L', 68),
            ('L', 30),
            ('R', 48),
            ('L', 5),
            ('R', 60),
            ('L', 55),
            ('L', 1),
            ('L', 99),
            ('R', 14),
            ('L', 82)
        ];
        let result = run_part_1(input);
        assert_eq!(result, 3);
    }
}

