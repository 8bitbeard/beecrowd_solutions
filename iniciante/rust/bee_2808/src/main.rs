use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    let (start, end) = (&input[..2].to_string(), &input[3..].to_string());

    println!(
        "{}",
        match calculate_possible_moves(start).contains(end) {
            true => "VALIDO",
            false => "INVALIDO",
        }
    );
}

fn convert_to_tuple(spot: &String) -> (i32, i32) {
    let chars = spot
        .chars()
        .map(|x| {
            if x.is_alphabetic() {
                (x as u8 - 96) as i32
            } else {
                (x as u8 - 48) as i32
            }
        })
        .collect::<Vec<i32>>();
    (chars[0], chars[1])
}

fn convert_to_string(spot: (i32, i32)) -> String {
    let mut a = ((spot.0 as u8 + 96) as char).to_string();
    let b = ((spot.1 as u8 + 48) as char).to_string();
    a.push_str(&b);
    a
}

fn calculate_possible_moves(spot: &String) -> Vec<String> {
    let (a, b) = convert_to_tuple(spot);
    let moves = vec![
        (a - 1, b - 2),
        (a - 1, b + 2),
        (a + 1, b - 2),
        (a + 1, b + 2),
        (a - 2, b - 1),
        (a - 2, b + 1),
        (a + 2, b - 1),
        (a + 2, b + 1),
    ];

    moves
        .into_iter()
        .filter(|x| x.0 > 0 && x.1 > 0)
        .map(|x| convert_to_string(x))
        .collect::<Vec<String>>()
}
