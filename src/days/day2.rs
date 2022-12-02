use crate::utils::read_input;

#[derive(Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Loser = 0,
    Draw = 3,
    Winner = 6,
}

fn outcome(opponent: &Choice, player: &Choice) -> Outcome {
    match opponent {
        Choice::Rock => match player {
            Choice::Rock => Outcome::Draw,
            Choice::Paper => Outcome::Winner,
            Choice::Scissors => Outcome::Loser,
        },
        Choice::Paper => match player {
            Choice::Rock => Outcome::Loser,
            Choice::Paper => Outcome::Draw,
            Choice::Scissors => Outcome::Winner,
        },
        Choice::Scissors => match player {
            Choice::Rock => Outcome::Winner,
            Choice::Paper => Outcome::Loser,
            Choice::Scissors => Outcome::Draw,
        },
    }
}

fn to_play(opponent: &Choice, outcome: Outcome) -> Choice {
    match outcome {
        Outcome::Draw => *opponent,
        Outcome::Loser => match opponent {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
        Outcome::Winner => match opponent {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
    }
}

fn convert(input: &str) -> Choice {
    match input {
        "A" => Choice::Rock,
        "X" => Choice::Rock,
        "B" => Choice::Paper,
        "Y" => Choice::Paper,
        "C" => Choice::Scissors,
        "Z" => Choice::Scissors,
        _ => panic!("Wrong choice: {}", input),
    }
}

pub fn run() -> (u32, u32) {
    let filename = "./resources/input/day2.txt";
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
    // println!("{:#?}", input);
    let mut points = 0;
    for game in input {
        if game.as_ref().unwrap().len() > 0 {
            let game_arr: Vec<&str> = game.as_ref().unwrap().split(" ").collect();
            let opponent = convert(game_arr[0]);
            let player = convert(game_arr[1]);
            let result = outcome(&opponent, &player);
            points += result as u32 + player as u32;
        }
    }
    points
}

fn result2(input: &Vec<Option<String>>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{read_input, result1, result2};

    #[test]
    fn test_result1() {
        let filename = "./resources/input/day2_sample.txt";

        let input: Vec<Option<String>> =
            read_input(filename).expect("Unable to read the input file");
        let result: u32 = result1(&input);
        assert_eq!(result, 15);
    }
}
