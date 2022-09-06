use std::io;

fn main() {

    let mut values = String::new();

    io::stdin().read_line(&mut values).expect("Failed to read line!");
    let values = values.trim();

    let vector: Vec<&str> = values.split(' ').collect();

    let a: f64 = vector[0].parse().unwrap();
    let b: f64 = vector[1].parse().unwrap();
    let c: f64 = vector[2].parse().unwrap();

    let inner_sqrt: f64 = f64::powf(b, 2.0) - 4.0 * a * c;

    if inner_sqrt < 0.0 || a == 0.0 {
        println!("Impossivel calcular");
        return;
    }

    let r1: f64 = (-b + inner_sqrt.sqrt()) / (2.0 * a);
    let r2: f64 = (-b - inner_sqrt.sqrt()) / (2.0 * a);

    println!("R1 = {:.5}", r1);
    println!("R2 = {:.5}", r2);

}
