use std::io;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let mut sides: Vec<f64> = input
                                .trim()
                                .split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect();
    
    if (sides[0] + sides[1] <= sides[2]) || (sides[1] + sides[2] <= sides[0]) || (sides[0] + sides[2] <= sides[1]) {
        let area: f64 = (sides[0] + sides[1]) * sides[2] / 2.0;
        println!("Area = {:.1}", area);
        return;
    }
    
    let perimetro: f64 = sides.iter().sum();
    println!("Perimetro = {:.1}", perimetro);
}
