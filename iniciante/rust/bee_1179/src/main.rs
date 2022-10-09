use std::io;

const SIZE: u32 = 15;

fn main() {
    let mut even: Vec<i32> = Vec::new();
    let mut odd: Vec<i32> = Vec::new();

    for _i in 1..=SIZE {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the line!");
        let input: i32 = input.trim().parse().unwrap();

        if input % 2 != 0 {
            if odd.len() < 5 {
                odd.push(input);
            } else {
                for i in 0..5 {
                    println!("impar[{}] = {}", i, odd[i]);
                }
                odd = Vec::new();
                odd.push(input);
            }
        } else {
            if even.len() < 5 {
                even.push(input);
            } else {
                for i in 0..5 {
                    println!("par[{}] = {}", i, even[i]);
                }
                even = Vec::new();
                even.push(input);
            }
        }
    }

    for (pos, e) in odd.iter().enumerate() {
        println!("impar[{}] = {}", pos, e);
    }
    for (pos, e) in even.iter().enumerate() {
        println!("par[{}] = {}", pos, e);
    }
}
