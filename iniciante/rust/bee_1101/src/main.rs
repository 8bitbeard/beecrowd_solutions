use std::io;

fn main() {
    let mut output_vec: Vec<Vec<i32>> = Vec::new();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let v: Vec<&str> = input.trim().split(' ').collect();
        let mut v: Vec<i32> = v.iter().map(|&x| x.parse::<i32>().unwrap()).collect();

        if v[0] <= 0 || v[1] <= 0 {
            break;
        }

        v.sort();

        output_vec.push(v);
    }

    for vector in output_vec {
        let mut total: i32 = 0;
        for i in vector[0]..=vector[1] {
            print!("{} ", i);
            total += i;
        }
        println!("Sum={}", total);
    }
}
