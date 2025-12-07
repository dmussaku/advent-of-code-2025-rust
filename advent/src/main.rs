mod days;

use days::{day1};
use days::{day2};
use days::{day3};


fn main() {
    let input = day1::main::read_input("src/days/day1/input_files/input.txt");
    let result = day1::main::run_part_1(input);
    println!("day 1 part 1 Result: {}", result);
    let input = day2::main::read_input("src/days/day2/input_files/input.txt");
    let result = day2::main::run_part_1(input);
    println!("day 2 part 1 Result: {}", result);
    let input = day3::main::read_input("src/days/day3/input_files/input.txt");
    let result = day3::main::run_part_1(input);
    println!("day 3 part 1 Result: {}", result);
}
