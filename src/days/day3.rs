use crate::utils::{alphabet, read_input};

pub fn run() -> (u32, u32) {
    let filename = "./resources/input/day3.txt";
    let input: Vec<Option<String>> = read_input(filename).expect("Unable to read the input file");

    (result1(&input), result2(&input))
}

/// Solve the first part of the daily challenge
///
/// Change the input type as required. It should be a vector, though. The input of the challenges
/// are usually text files with one integer value per line.
///
/// Use the sample input and result in the test.
fn result1(input: &Vec<Option<String>>) -> u32 {
    let mut global_common: Vec<char> = vec![];
    let mut common: Vec<char> = vec![];
    for s in input {
        let chars: Vec<char> = s.as_ref().unwrap().chars().collect();
        if chars.len() > 0 {
            let rucksack: Vec<&[char]> = chars.chunks(chars.len() / 2).collect();
            // println!("{:#?}", rucksack);
            let compartment1 = rucksack[0];
            let compartment2 = rucksack[1];

            for item1 in compartment1 {
                for item2 in compartment2 {
                    if item1 == item2 && !common.contains(item1) {
                        common.push(*item1);
                    }
                }
            }
            // println!("{:#?}", common);
            global_common.append(&mut common);
        }
    }
    let mut sum = 0;
    for letter in global_common {
        sum += weight(letter);
    }
    sum.into()
}

fn weight(letter: char) -> u8 {
    let index: u8 = alphabet()
        .iter()
        .position(|&r| r == letter)
        .unwrap()
        .try_into()
        .unwrap();
    index + 1
}

fn result2(input: &Vec<Option<String>>) -> u32 {
    0
}

#[derive(Debug, PartialEq, Eq)]
struct Letter {
    letter: char,
    value: u8,
}

fn letters_weighted() -> Vec<Letter> {
    let alpha = alphabet();
    let mut letters: Vec<Letter> = vec![];
    for (i, letter) in alpha.iter().enumerate() {
        letters.push(Letter {
            letter: *letter,
            value: (i + 1) as u8,
        });
    }
    letters
}

#[cfg(test)]
mod tests {
    use super::{letters_weighted, read_input, result1, result2, Letter};

    #[test]
    fn test_letters_weighted() {
        let letters = letters_weighted();

        let first = Letter {
            letter: 'a',
            value: 1,
        };
        let last = Letter {
            letter: 'Z',
            value: 52,
        };

        assert_eq!(letters[0], first);
        assert_eq!(letters[51], last);
    }

    #[test]
    fn test_result1() {
        let filename = "./resources/input/day3_sample.txt";

        let input: Vec<Option<String>> =
            read_input(filename).expect("Unable to read the input file");
        let result: u32 = result1(&input);
        assert_eq!(result, 157);
    }
}
