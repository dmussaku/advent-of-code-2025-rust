use std::fs;


pub fn read_input(path: &str) -> Vec<(String, String)>{
    let contents = fs::read_to_string(path)
        .expect("Something went wrong with a file");
    contents.split(',')
        .map(|line| {
            let mut parts = line.split('-');
            let range1 = parts.next().unwrap().to_string();
            let range2 = parts.next().unwrap().to_string();
            (range1, range2)
        })
        .collect()
}

pub fn find_invalid_ids(start: u64, end: u64) -> Vec<u64> {
    let mut invalid_ids = Vec::new();
    for id in start..=end {
        let id_str = id.to_string();
        let id_str_len = id_str.len();
        if id_str_len % 2 != 0 {
            // base case: odd length cannot be checked
            continue
        }
        if id_str[0..id_str_len/2] == id_str[id_str_len/2..id_str_len] {
            // println!("Invalid ID found: {}", id);
            invalid_ids.push(id);
        }
    }
    invalid_ids
}


pub fn run_part_1(input: Vec<(String, String)>) -> u64 {
    let mut sum = 0;
    for (start_id, end_id) in input{
        println!("Checking range: {}-{}", start_id, end_id);
        let start = start_id.parse::<u64>().unwrap();
        let end = end_id.parse::<u64>().unwrap();
        let invalid_ids = find_invalid_ids(start, end);
        for invalid_id in invalid_ids {
            sum += invalid_id as u64;
        }
    }
    sum
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
    fn test_find_invalid_ids() {
        let result = find_invalid_ids(11, 22);
        assert_eq!(result, vec![11, 22]);
    }

    #[test]
    fn test_run_part_1() {
        let input = read_input("src/days/day2/input_files/test_input.txt");
        let result = run_part_1(input);
        assert_eq!(result, 1227775554);
    }
}

