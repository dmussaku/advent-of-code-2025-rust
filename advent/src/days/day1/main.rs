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

pub fn rotate_with_clicks(direction: &char, current_position: usize, rotate_by: usize) -> (usize, usize) {
    match direction {
        'R' => {
            let new_position = (current_position + rotate_by) % 100;
            let clicks = (current_position + rotate_by) / 100;
            (new_position, clicks)
        },
        'L' => {
            let total = current_position as i32 - rotate_by as i32;
            let new_position = total.rem_euclid(100).abs() as usize;
            let clicks = if total < 0 {
                (rotate_by - current_position + 99) / 100
            } else {
                0
            };
            (new_position, clicks)
        },
        _ => (current_position, 0),
    }
}

pub fn run_part_1(input: Vec<(char, usize)>) -> usize {
    let mut final_position = 50;
    let mut num_of_0_pos: usize = 0;
    for (direction, number) in input{
        // let last_position = final_position;
        // println!("Rotating from position: {}, direction: {}, by: {}", final_position, direction, number);
        // final_position = rotate(&direction, final_position, number);
        (final_position, _) = rotate_with_clicks(&direction, final_position, number);
        // println!("Rotation complete! Current position: {}, moved to: {}, by {}; result = {}", last_position, direction, number, final_position);
        if final_position == 0 {
            num_of_0_pos += 1;
        }
    }
    num_of_0_pos
}

// pub fn run_part_2(input: Vec<(char, usize)>) -> usize {
//     let mut final_position = 50;
//     let mut total_clicks = 0;
//     let mut num_of_0_pos: usize = 0;
//     for (direction, number) in input{
//         // let last_position = final_position;
//         // println!("Rotating from position: {}, direction: {}, by: {}", final_position, direction, number);
//         // final_position = rotate(&direction, final_position, number);
//         let res = rotate_with_clicks(&direction, final_position, number);
//         final_position = res.0;
//         total_clicks += res.1;
//         // println!("Rotation complete! Current position: {}, moved to: {}, by {}; result = {}", last_position, direction, number, final_position);
//         if final_position == 0 {
//             num_of_0_pos += 1;
//         }
//     }
//     num_of_0_pos + total_clicks
// }


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
    fn test_rotate_with_clicks() {
        assert_eq!(rotate_with_clicks(&'R', 0, 10), (10, 0));
        assert_eq!(rotate_with_clicks(&'L', 10, 5), (5, 0));
        assert_eq!(rotate_with_clicks(&'R', 95, 10), (5, 1));
        assert_eq!(rotate_with_clicks(&'L', 5, 10), (95, 1));
        assert_eq!(rotate_with_clicks(&'L', 5, 101), (4, 1));
        assert_eq!(rotate_with_clicks(&'R', 5, 201), (6, 2));
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

