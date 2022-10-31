use std::io;

const PI: f64 = 3.14;

fn main() {
    loop {
        let mut volume = String::new();
        io::stdin()
            .read_line(&mut volume)
            .expect("Failed to read the line!");
        let volume: f64 = match volume.trim().parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        let mut diameter = String::new();
        io::stdin()
            .read_line(&mut diameter)
            .expect("Failed to read the line!");
        let diameter: f64 = match diameter.trim().parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        let radius: f64 = diameter / 2.0;
        let base_area = PI * radius * radius;

        let height = volume / base_area;

        println!("ALTURA = {:.2}", height);
        println!("AREA = {:.2}", base_area);
    }
}
