fn main() {
    let mut total: f64 = 1.0;
    let mut j: f64 = 2.0;

    for n in (3..=39).step_by(2) {
        total += (n as f64) / j;
        j = j * 2.0;
    }

    println!("{:.2}", total);
}
