pub fn run(input: Vec<Option<u32>>) -> (u32, u32) {
    (result1(&input), result2(&input))
}

/// Solve the first part of the daily challenge
///
/// Change the input type as required. It should be a vector, though. The input of the challenges
/// are usually text files with one integer value per line.
///
/// Use the sample input and result in the test.
fn result1(input: &Vec<u32>) -> u32 {
    let mut bigger = 0;
    let mut previous = 99999;
    for num in input {
        if num > &previous {
            bigger += 1;
        }
        previous = *num;
    }

    bigger
}

fn result2(input: &Vec<u32>) -> u32 {
    let mut bigger = 0;
    let mut previous = 99999;
    for (i, num) in input.iter().enumerate() {
        let mut current = *num;
        if i + 1 < input.len() {
            current = current + input[i + 1];
        }
        if i + 2 < input.len() {
            current = current + input[i + 2];
        }
        if current > previous {
            bigger += 1;
        }
        previous = current;
    }

    bigger
}

#[cfg(test)]
mod tests {
    use crate::days::example::{result1, result2};

    #[test]
    fn test_result1() {
        let test_input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let result = result1(&test_input);
        assert_eq!(result, 7);
    }
    #[test]
    fn test_result2() {
        let test_input = vec![607, 618, 618, 617, 647, 716, 769, 792];

        let result = result2(&test_input);
        assert_eq!(result, 5);
    }
}
