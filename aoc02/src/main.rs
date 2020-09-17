use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    part_one(&input)?;
    part_two(&input)?;
    Ok(())
}

fn part_one(input: &str) -> Result<(), Box<dyn Error>> {
    let mut two_count = 0;
    let mut three_count = 0;

    for line in input.lines() {
        let mut freq_map = HashMap::new();

        for c in line.chars() {
            let count = freq_map.entry(c).or_insert(0);
            *count += 1;
        }

        let mut has_two = false;
        let mut has_three = false;

        for (_, value) in &freq_map {
            if value == &2 {
                has_two = true;
            } else if value == &3 {
                has_three = true;
            }
        }

        if has_two {
            two_count += 1;
        }

        if has_three {
            three_count += 1;
        }
    }

    println!("{}", two_count * three_count);

    Ok(())
}

fn part_two(input: &str) -> Result<(), Box<dyn Error>> {
    let ids: Vec<&str> = input.lines().collect();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            if let Some(common) = common_letters(&ids[i], &ids[j]) {
                println!("{}", common);
                return Ok(());
            }
        }
    }

    Err(From::from("could not find two correct box ids"))
}

fn common_letters(id1: &str, id2: &str) -> Option<String> {
    let mut mismatch = false;

    for (c1, c2) in id1.chars().zip(id2.chars()) {
        if c1 != c2 {
            if mismatch {
                return None;
            } else {
                mismatch = true;
            }
        }
    }

    Some(
        id1.chars()
            .zip(id2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}
