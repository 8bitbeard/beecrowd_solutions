use std::io;

fn main() {

    let mut tests = String::new();
    io::stdin().read_line(&mut tests).expect("Failed to read line!");
    let tests = match tests.trim().parse::<u32>() {
        Ok(x) => x,
        Err(_) => return,
    };

    for _i in 0..tests {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let radius: Vec<u32> = input.trim().split(' ').map(|x| x.parse::<u32>().unwrap()).collect();

        println!("{}", radius.iter().sum::<u32>());
    }
}
