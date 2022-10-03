fn main() {
    let mut total: f64 = 0.0;

    for n in 1..=100 {
        total += 1.0 / (n as f64);
    }

    println!("{:.2}", total);
}
