use std::io;
use std::collections::HashMap;


fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();
    
    let mut ddds = HashMap::new();
    ddds.insert("61", "Brasilia");
    ddds.insert("71", "Salvador");
    ddds.insert("11", "Sao Paulo");
    ddds.insert("21", "Rio de Janeiro");
    ddds.insert("32", "Juiz de Fora");
    ddds.insert("19", "Campinas");
    ddds.insert("27", "Vitoria");
    ddds.insert("31", "Belo Horizonte");
    
    let output = match ddds.get(input) {
        Some(x) => x,
        None => {
            println!("DDD nao cadastrado");
            return;
        }
    };
    
    println!("{}", output);
}
