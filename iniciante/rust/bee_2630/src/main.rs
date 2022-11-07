use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: usize = tests.trim().parse::<usize>().unwrap();

    for i in 1..=tests {
        let mut strategy = String::new();
        io::stdin()
            .read_line(&mut strategy)
            .expect("Failed to read strategy");
        let strategy = strategy.trim();

        let mut rgb = String::new();
        io::stdin()
            .read_line(&mut rgb)
            .expect("Failed to read rbg values");
        let mut rgb: Vec<u32> = rgb
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let (r, g, b): (f64, f64, f64) = (rgb[0] as f64, rgb[1] as f64, rgb[2] as f64);
        rgb.sort();

        let output: f64 = match strategy {
            "min" => rgb[0] as f64,
            "max" => rgb[2] as f64,
            "mean" => rgb.iter().sum::<u32>() as f64 / 3.0,
            _ => 0.3 * r + 0.59 * g + 0.11 * b,
        };

        println!("Caso #{}: {}", i, output as u32);
    }
}
