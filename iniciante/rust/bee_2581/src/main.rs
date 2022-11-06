use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    let input: usize =  input.trim().parse::<usize>().unwrap();

    for _i in 0..input {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read the line");

        println!("I am Toorg!");
    }
}
