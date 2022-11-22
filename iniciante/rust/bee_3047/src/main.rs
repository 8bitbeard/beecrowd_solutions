use std::io;

fn main() {
    let mut output = String::new();
    io::stdin()
        .read_line(&mut output)
        .expect("Failed to read the output");
    let output = output.trim().parse::<u32>().unwrap();

    let mut ages: Vec<u32> = vec![output];
    for _i in 0..2 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim().parse::<u32>().unwrap();
        ages[0] -= input;
        ages.push(input);
    }

    ages.sort();

    println!("{}", ages[2]);
}
