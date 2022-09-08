use std::io;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let point_vector: Vec<f64> = input
                                    .trim()
                                    .split(' ')
                                    .map(|s| s.parse().unwrap())
                                    .collect();
    
    if point_vector[0] == 0.0 && point_vector[1] == 0.0 {
        println!("Origem");
    } else if point_vector[0] == 0.0 {
        println!("Eixo Y");
    } else if point_vector[1] == 0.0 {
        println!("Eixo X");
    } else if point_vector[0] > 0.0 && point_vector[1] > 0.0 {
        println!("Q1");
    } else if point_vector[0] > 0.0 && point_vector[1] < 0.0 {
        println!("Q4");
    } else if point_vector[0] < 0.0 && point_vector[1] < 0.0 {
        println!("Q3");
    } else {
        println!("Q2");
    };
}
