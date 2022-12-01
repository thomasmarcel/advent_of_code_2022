# Advent of Code 2022

The yearly attempt at consistency with coding as a hobby!

## Structure

- `src/main.rs`: only running commands from the library
- `src/lib.rs`: importing daily challenges and utils
- `src/utils.rs`: helpers to read the input file, adn other functions
- `resources/input/`: where the input files are located
- `src/days/day*.rs`: daily challenge sokution

## Test

- Each helper is tested
- Tests are made from the daily puzzle examples to test the puzzle solution on a known result

### Run

- `cargo test -- --nocapture` to run the test suite

## New Day Puzzle

To start a new daily puzzle:

### Boilerplate

1. Write the input example in `resources/input`
1. Write the input in `resources/input`
1. Copy a previous day file in `src/days`
1. Import the new file as a module in `src/days/mod.rs`
1. Add it and run it in `src/main.rs`

### Puzzle Solving (Test Driven Development)

Now in your daily puzzle file:

1. Remove all that is result2 and test_result2
1. In test_result1, add the correct sample input and the expectation described in the puzzle instructions
1. in the result1 function, remove the content and return 0
1. Run `cargo test -- --nocapture` the test should fail
1. Now, solve the puzzle in result1
1. Test against the sample input and result again
1. Fix and improve until the test passes
1. Run the actual data with `cargo run`
1. Send your answer
1. Repeat for the 2nd puzzle
