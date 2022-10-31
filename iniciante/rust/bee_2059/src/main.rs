use std::io;

fn main() {
    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read the line!");
    let values: Vec<u32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let v = vec![
        convert(values[0] == 1),
        convert((values[1] + values[2]) % 2 == 0),
        convert(values[3] == 1),
        convert(values[4] == 1),
    ];
    let binary = v.join("");
    let intval = isize::from_str_radix(&binary[..], 2).unwrap();

    match intval {
        3 | 4 | 7 | 8 | 11 | 15 => println!("Jogador 2 ganha!"),
        _ => println!("Jogador 1 ganha!"),
    }
}

fn convert(value: bool) -> String {
    let s = if value { "1" } else { "0" };
    s.to_string()
}
