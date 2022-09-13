use std::io;

fn main() {

    let mut v: Vec<i32> = Vec::new();
    let mut counter: u8 = 0;

    while counter < 5 {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: i32 = input.trim().parse().unwrap();

        v.push(input);

        counter += 1;
    };

    v.retain(|&i| i % 2 == 0);

    println!("{} valores pares", v.len());
}
