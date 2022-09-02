use std::io;

const PI: f64 = 3.14159;

fn main() {

    let mut input_data = String::new();

    io::stdin().read_line(&mut input_data).expect("Failed to read line!");
    let input_data = input_data.trim();
    let data_vector: Vec<&str> = input_data.split(' ').collect();

    let a: f64 = data_vector[0].parse().unwrap();
    let b: f64 = data_vector[1].parse().unwrap();
    let c: f64 = data_vector[2].parse().unwrap();

    let triangle_area: f64 = a * c / 2.0;
    let circle_area: f64 = PI * c * c;
    let trapezium_area: f64 = (a + b) * c / 2.0;
    let square_area: f64 = b * b;
    let rectangle_area: f64 = a * b;

    println!("TRIANGULO: {:.3}", triangle_area);
    println!("CIRCULO: {:.3}", circle_area);
    println!("TRAPEZIO: {:.3}", trapezium_area);
    println!("QUADRADO: {:.3}", square_area);
    println!("RETANGULO: {:.3}", rectangle_area);
}
