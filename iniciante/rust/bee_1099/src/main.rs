use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin().read_line(&mut amount).expect("Failed to read line!");
    let amount: u32 = amount.trim().parse().unwrap();

    let mut output_vec: Vec<u32> = Vec::new();

    for _i in 0..amount {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let v: Vec<&str> = input.trim().split(' ').collect();
        let mut v: Vec<u32> = v.iter().map(|&x| x.parse::<u32>().unwrap()).collect();
        v.sort();

        let mut total: u32 = 0;
        for j in (v[0] + 1)..v[1] {
            if j % 2 != 0 { total += j }
        }

        output_vec.push(total);
    }

    for number in output_vec {
        println!("{}", number);
    }
}
