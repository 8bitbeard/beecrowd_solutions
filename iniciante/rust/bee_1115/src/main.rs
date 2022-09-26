use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let v: Vec<&str> = input.trim().split(' ').collect();
        let v: Vec<i32> = v.iter().map(|&x| x.parse::<i32>().unwrap()).collect();

        if v[0] == 0 || v[1] == 0 {
            break;
        }

        if v[0] > 0 && v[1] > 0 {
            println!("primeiro");
        } else if v[0] < 0 && v[1] > 0 {
            println!("segundo");
        } else if v[0] < 0 && v[1] < 0 {
            println!("terceiro");
        } else {
            println!("quarto");
        }
    }

}
