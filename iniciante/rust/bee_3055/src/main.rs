use std::io;

fn main() {
    let mut grade_a = String::new();
    io::stdin()
        .read_line(&mut grade_a)
        .expect("Failed to read the grade_a");
    let grade_a: u32 = grade_a.trim().parse::<u32>().unwrap();

    let mut average = String::new();
    io::stdin()
        .read_line(&mut average)
        .expect("Failed to read the average");
    let average: u32 = average.trim().parse::<u32>().unwrap();

    println!("{}", average * 2 - grade_a);
}
