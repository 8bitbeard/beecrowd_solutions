use std::io;

fn main() {
    let mut data = String::new();
    io::stdin()
        .read_line(&mut data)
        .expect("Failed to read the line!");
    let data: Vec<u32> = data
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (mut start, actions) = (data[0], data[1]);

    for _i in 0..actions {
        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read the line!");
        let action = action.trim();

        match action {
            "fechou" => start += 1,
            "clicou" => start -= 1,
            _ => continue,
        }
    }
    println!("{}", start);
}
