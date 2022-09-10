use std::io;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
    
    let salary: f64 = match input.trim().parse() {
        Ok(x) => x,
        Err(_) => {
            println!("Valor incorreto!");
            return;
        }
    };
    
    let (reajuste, percentual): (f64, String) = match salary {
        (0.0..=400.0) => (salary * 0.15, String::from("15 %")),
        (400.01..=800.0) => (salary * 0.12, String::from("12 %")),
        (800.01..=1200.0) => (salary * 0.10, String::from("10 %")),
        (1200.01..=2000.0) => (salary * 0.07, String::from("7 %")),
        _ => (salary * 0.04, String::from("4 %")),
    };
    
    println!("Novo salario: {:.2}", salary + reajuste);
    println!("Reajuste ganho: {:.2}", reajuste);
    println!("Em percentual: {}", percentual);

}
