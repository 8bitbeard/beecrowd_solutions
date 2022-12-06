use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line!");
    let input = input.trim();
    let number = input.parse::<f64>().unwrap();

    if number == 0.0 && input.contains("-") {
        println!("-0.0000E+00");
        return;
    }

    let sn = format!("{:.4E}", number);
    let v: Vec<f64> = sn.split("E").map(|x| x.parse::<f64>().unwrap()).collect();

    println!("{:+.4}E{:+03}", v[0], v[1] as i32);
}
