use std::io;

fn main() {

    let mut point_a = String::new();
    let mut point_b = String::new();

    io::stdin().read_line(&mut point_a).expect("Failed to read line!");
    let point_a = point_a.trim();

    io::stdin().read_line(&mut point_b).expect("Failed to read line!");
    let point_b = point_b.trim();

    let vector_point_a: Vec<&str> = point_a.split(' ').collect();
    let vector_point_b: Vec<&str> = point_b.split(' ').collect();

    let point_a_x: f64 = vector_point_a[0].parse().unwrap();
    let point_a_y: f64 = vector_point_a[1].parse().unwrap();
    let point_b_x: f64 = vector_point_b[0].parse().unwrap();
    let point_b_y: f64 = vector_point_b[1].parse().unwrap();

    let distance: f64 = (f64::powf(point_b_x - point_a_x, 2.0) + f64::powf(point_b_y - point_a_y, 2.0)).sqrt();

    println!("{:.4}", distance);
}
