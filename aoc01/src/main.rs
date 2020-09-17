use std::fs;

fn main() {
    const FILE_NAME: &str = "frequencies.txt";

    let mut count = 0;
    let contents = fs::read_to_string(FILE_NAME).unwrap();

    for line in contents.lines() {
        let sign = &line[0..1];
        let mut number: i32 = line[1..].parse().unwrap();

        if sign == "-" {
            number *= -1;
        }

        count += number
    }

    println!("{}", count);
}
