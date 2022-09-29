use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");

    let numbers: Vec<&str> = input.trim().split(' ').collect();
    let numbers: Vec<u32> = numbers.iter().map(|&x| x.parse::<u32>().unwrap()).collect();

    for number in (numbers[0]..=numbers[1]).step_by(numbers[0] as usize) {
        let mut v: Vec<String> = ((number - numbers[0] + 1)..=number).map(|x| x.to_string()).collect();

        let joined = v.join(" ");
        println!("{}", joined);
    }
}
