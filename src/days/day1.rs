use crate::utils::read_input;

pub fn run() -> (u32, u32) {
    let filename = "./resources/input/day1.txt";
    let input: Vec<Option<u32>> = read_input(filename).expect("Unable to read the input file");

    (result1(&input), result2(&input))
}

/// Solve the first part of the daily challenge
///
/// Change the input type as required. It should be a vector, though. The input of the challenges
/// are usually text files with one integer value per line.
///
/// Use the sample input and result in the test.
fn result1(input: &Vec<Option<u32>>) -> u32 {
    let mut elves: Vec<u32> = vec![];
    let mut sum = 0;
    let mut max = 0;

    for cal in input {
        match cal {
            Some(c) => sum += c,
            None => {
                elves.push(sum);
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        }
    }
    max
}

fn result2(input: &Vec<Option<u32>>) -> u32 {
    let mut elves: Vec<u32> = vec![];
    let mut sum = 0;
    let mut max = 0;

    for cal in input {
        match cal {
            Some(c) => sum += c,
            None => {
                elves.push(sum);
                if sum > max {
                    max = sum;
                }
                sum = 0;
            }
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::{read_input, result1, result2};

    #[test]
    fn test_result1() {
        let filename = "./resources/input/day1_sample.txt";

        let input: Vec<Option<u32>> = read_input(filename).expect("Unable to read the input file");
        let result: u32 = result1(&input);
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_result2() {
        let filename = "./resources/input/day1_sample.txt";

        let input: Vec<Option<u32>> = read_input(filename).expect("Unable to read the input file");
        let result: u32 = result2(&input);
        assert_eq!(result, 45000);
    }
}
