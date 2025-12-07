use std::fs;


pub fn read_input(path: &str) -> Vec<(String, String)>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    // Each line contains a pair of ranges in the format "1-10,5-15,500-1000" what i need is [(1,10), (5,15), (500,1000)]
    contents.lines()
        .map(|line| {
            let mut parts = line.split(',');
            let range1 = parts.next().unwrap().to_string();
            let range2 = parts.next().unwrap().to_string();
            (range1, range2)
        })
        .collect()
}

pub fn run_part_1(input: Vec<(String, String)>) -> usize {
    input.len();
    0
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_read_input() {
        let result = read_input("src/days/day2/input_files/test_input.txt");
        assert_eq!(result, 
            vec![
                ("11".to_owned(), "22".to_owned()),
                ("95".to_owned(), "115".to_owned()),
                ("998".to_owned(), "1012".to_owned()),
                ("1188511880".to_owned(), "1188511890".to_owned()),
                ("222220".to_owned(), "222224".to_owned()),
                ("1698522".to_owned(), "1698528".to_owned()),
                ("446443".to_owned(), "446449".to_owned()),
                ("38593856".to_owned(), "38593862".to_owned()),
                ("565653".to_owned(), "565659".to_owned()),
                ("824824821".to_owned(), "824824827".to_owned()),
                ("2121212118".to_owned(), "2121212124".to_owned())
            ]
        );
    }

    #[test]
    fn test_run_part_1() {
        let input = read_input("src/days/day2/input_files/test_input.txt");
        let result = run_part_1(input);
        assert_eq!(result, 0);
    }
}

