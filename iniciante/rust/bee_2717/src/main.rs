use std::io;

fn main() {
    let mut total = String::new();
    io::stdin()
        .read_line(&mut total)
        .expect("Failed to read_line");
    let total: u32 = match total.trim().parse::<u32>() {
        Ok(x) => x,
        Err(_) => return,
    };

    let mut times = String::new();
    io::stdin()
        .read_line(&mut times)
        .expect("Failed to read the line");
    let times: Vec<u32> = times
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    match times.iter().sum::<u32>() <= total {
        true => println!("Farei hoje!"),
        false => println!("Deixa para amanha!"),
    }
}
