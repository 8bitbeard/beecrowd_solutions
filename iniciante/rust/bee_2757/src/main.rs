use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read the line");
    let a: Vec<f64> = a.trim().split(' ').map(|x| x.parse::<f64>().unwrap()).collect();

    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read the line");
    let b: Vec<f64> = b.trim().split(' ').map(|x| x.parse::<f64>().unwrap()).collect();

    println!("A = {:.6}, B = {:.6}", a[0], a[1]);

}
