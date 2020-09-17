use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    const FILE_NAME: &str = "frequencies.txt";

    let contents = fs::read_to_string(FILE_NAME).unwrap();
    part_one(&contents)?;
    part_two(&contents)?;

    Ok(())
}

fn part_one(contents: &str) -> Result<(), Box<dyn Error>> {
    let mut freq = 0;

    for line in contents.lines() {
        let number: i32 = line.parse()?;
        freq += number;
    }

    println!("{}", freq);
    Ok(())
}

fn part_two(contents: &str) -> Result<(), Box<dyn Error>> {
    let mut freq = 0;
    let mut seen = HashSet::new();

    loop {
        for line in contents.lines() {
            let number: i32 = line.parse()?;
            freq += number;

            if seen.contains(&freq) {
                println!("{}", freq);
                return Ok(());
            }

            seen.insert(freq);
        }
    }
}
