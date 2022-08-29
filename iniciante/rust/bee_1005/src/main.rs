use std::io;

fn main() {
    let mut grade_a = String::new();
    let mut grade_b = String::new();

    io::stdin()
        .read_line(&mut grade_a)
        .expect("Failed to read line!");

    let grade_a: f64 = grade_a.trim().parse().unwrap();

    io::stdin()
        .read_line(&mut grade_b)
        .expect("Failed to read line!");

    let grade_b: f64 = grade_b.trim().parse().unwrap();

    let media: f64 = (3.5 * grade_a + 7.5 * grade_b) / 11.0;

    println!("MEDIA = {:.5}", media);
}
