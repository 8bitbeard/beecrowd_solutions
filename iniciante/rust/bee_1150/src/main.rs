use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    let x: i32 = input.trim().parse().unwrap();
    let mut z: i32 = 0;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let input: i32 = input.trim().parse().unwrap();

        if input > x {
            z = input;
            break;
        }
    }

    let mut counter: i32 = 1;
    let mut buffer: i32 = x;

    while buffer <= z {
        buffer += x + counter;
        counter += 1;
    }

    println!("{}", counter);
}
