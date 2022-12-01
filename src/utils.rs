use std::fs;
use std::io::Error;

pub fn read_input<T: std::str::FromStr>(filename: &str) -> Result<Vec<T>, Error> {
    let data = fs::read_to_string(filename).unwrap();
    let input = data.split("\n");
    let mut output: Vec<T> = Vec::new();
    for line in input {
        if line.len() > 0 {
            let parsed_line = line.parse::<T>();
            match parsed_line {
                Ok(l) => output.push(l),
                Err(_e) => (),
            }
        }
    }

    Ok(output)
}
