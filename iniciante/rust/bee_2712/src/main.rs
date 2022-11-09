use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read_line");
    let tests: usize = tests.trim().parse().unwrap();

    for _i in 0..tests {
        let mut plate = String::new();
        io::stdin()
            .read_line(&mut plate)
            .expect("Failed to read line");
        let plate = plate.trim().to_string();

        match validate_plate(&plate) {
            true => println!("{}", match_day(plate)),
            false => println!("FAILURE"),
        };
    }
}

fn validate_plate(plate: &String) -> bool {
    let chars = plate.chars().collect::<Vec<char>>();
    if chars.len() != 8 {
        return false;
    }
    for c in &chars[..=2] {
        if !c.is_uppercase() || !c.is_alphabetic() {
            return false;
        }
    }
    if chars[3] != '-' {
        return false;
    }
    for c in &chars[4..] {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn match_day(plate: String) -> String {
    let number = plate
        .chars()
        .last()
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap();
    let output = match number {
        1 | 2 => "MONDAY",
        3 | 4 => "TUESDAY",
        5 | 6 => "WEDNESDAY",
        7 | 8 => "THURSDAY",
        _ => "FRIDAY",
    };

    output.to_string()
}
