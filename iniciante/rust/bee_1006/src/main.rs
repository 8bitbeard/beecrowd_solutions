use std::io;

fn main() {
    let mut grade_a = String::new();
    let mut grade_b = String::new();
    let mut grade_c = String::new();

    io::stdin()
        .read_line(&mut grade_a)
        .expect("Failed to read line!");
    let grade_a: f64 = grade_a.trim().parse().unwrap();

    io::stdin()
        .read_line(&mut grade_b)
        .expect("Failed to read line!");
    let grade_b: f64 = grade_b.trim().parse().unwrap();

    io::stdin()
        .read_line(&mut grade_c)
        .expect("Failed to read line!");
    let grade_c: f64 = grade_c.trim().parse().unwrap();

    let media: f64 = (2.0 * grade_a + 3.0 * grade_b + 5.0 * grade_c) / 10.0;

    println!("MEDIA = {:.1}", media);
}
