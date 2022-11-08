use std::io;

fn main() {
    let mut floors: Vec<u32> = Vec::new();

    loop {
        let mut employees = String::new();
        io::stdin()
            .read_line(&mut employees)
            .expect("Failed to read line");
        match employees.trim() {
            "" => break,
            x => floors.push(x.parse::<u32>().unwrap()),
        };
    }

    let mut times: Vec<u32> = Vec::new();
    for pos in 0..floors.len() {
        let mut idx: i32 = 0;
        let s = floors.iter().fold(0u32, |sum, val| {
            idx += 1;
            sum + (val * (idx - pos as i32 - 1).abs() as u32 * 2)
        });

        times.push(s);
    }

    println!("{}", times.iter().min().unwrap());
}
