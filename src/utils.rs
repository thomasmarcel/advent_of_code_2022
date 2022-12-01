use std::fs;
use std::io::Error;

pub fn read_input<T: std::str::FromStr>(filename: &str) -> Result<Vec<Option<T>>, Error> {
    let data = fs::read_to_string(filename).expect("Unable to read the input file");
    let input = data.split("\n");
    let mut output: Vec<Option<T>> = Vec::new();
    for line in input {
        let parsed_line = line.parse::<T>();
        match parsed_line {
            Ok(l) => output.push(Some(l)),
            Err(_e) => output.push(None),
        }
    }

    Ok(output)
}

pub fn top(n: u32, arr: Vec<u32>) -> Vec<u32> {
    let mut t: Vec<u32> = (0..n).map(|_| 0).collect();
    for n in arr {
        let t2 = t.clone();
        for (i, u) in t2.iter().enumerate() {
            if n >= *u {
                t.insert(i, n);
                t.pop();
                break;
            }
        }
    }

    t
}

#[cfg(test)]
mod tests {
    use super::{read_input, top};

    #[test]
    fn test_read_input() {
        let filename = "./resources/input/day1_sample.txt";

        let result: Vec<Option<u32>> = read_input(filename).expect("Unable to read the input file");

        let expected = vec![
            Some(1000),
            Some(2000),
            Some(3000),
            None,
            Some(4000),
            None,
            Some(5000),
            Some(6000),
            None,
            Some(7000),
            Some(8000),
            Some(9000),
            None,
            Some(10000),
            None,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_top() {
        let input = vec![1, 5, 7, 3, 4, 9, 2, 4, 8, 5, 2, 3, 6, 8];
        let expected = [9, 8, 8];
        let result = top(3, input);

        assert_eq!(result, expected);
    }
}
