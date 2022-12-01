use advent_of_code_2022::days;
use advent_of_code_2022::utils::read_input;

fn main() {
    let data: Vec<u32> = read_input("./resources/input/example.txt").unwrap();
    let result = days::example::run(data);
    println!("Example: {:?}", result);
}
