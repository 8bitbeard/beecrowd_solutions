use std::io;

fn main() {
    let mut counter: (i32, i32) = (0, 0);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let (action, qty) = match input.trim() {
            "ABEND" => break,
            _ => (&input[..6], input[7..].trim().parse::<i32>().unwrap()),
        };

        match action {
            "SALIDA" => {
                counter.0 += 1;
                counter.1 += qty;
            }
            _ => {
                counter.0 -= 1;
                counter.1 -= qty;
            }
        }
    }
    println!("{}", counter.1);
    println!("{}", counter.0);
}
