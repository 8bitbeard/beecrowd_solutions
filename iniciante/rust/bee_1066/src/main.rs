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

    let mut output: Vec<i32> = vec![0, 0, 0 ,0];

    for number in v {
        if number % 2 == 0 { output[0] += 1 }
        if number % 2 != 0 { output[1] += 1 }
        if number > 0 { output[2] += 1 }
        if number < 0 { output[3] += 1 }
    }

    println!("{} valor(es) par(es)", output[0]);
    println!("{} valor(es) impar(es)", output[1]);
    println!("{} valor(es) positivo(s)", output[2]);
    println!("{} valor(es) negativo(s)", output[3]);
}
