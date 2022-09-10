use std::io;
use std::collections::HashMap;


fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin()
        .read_line(&mut a)
        .expect("failed to read line!");
    io::stdin()
        .read_line(&mut b)
        .expect("failed to read line!");
    io::stdin()
        .read_line(&mut c)
        .expect("failed to read line!");
    
    let a = a.trim();
    let b = b.trim();
    let c = c.trim();
    
    let mut hash_a: HashMap<&str, &str> = HashMap::new();
    hash_a.insert("carnivoro", "aguia");
    hash_a.insert("onivoro", "pomba");
    let mut hash_b: HashMap<&str, &str> = HashMap::new();
    hash_b.insert("onivoro", "homem");
    hash_b.insert("herbivoro", "vaca");
    let mut hash_c: HashMap<&str, &str> = HashMap::new();
    hash_c.insert("hematofago", "pulga");
    hash_c.insert("herbivoro", "lagarta");
    let mut hash_d: HashMap<&str, &str> = HashMap::new();
    hash_d.insert("hematofago", "sanguessuga");
    hash_d.insert("onivoro", "minhoca");
    let mut hash_e: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    hash_e.insert("ave", hash_a);
    hash_e.insert("mamifero", hash_b);
    let mut hash_f: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    hash_f.insert("inseto", hash_c);
    hash_f.insert("anelideo", hash_d);
    let mut graph: HashMap<&str, HashMap<&str, HashMap<&str, &str>>> = HashMap::new();
    graph.insert("vertebrado", hash_e);
    graph.insert("invertebrado", hash_f);
    
    let output = match graph.get(&a) {
        Some(hash_one) => {
            match hash_one.get(&b) {
                Some(hash_two) => {
                    match hash_two.get(&c) {
                        Some(result) => result,
                        _ => {
                                println!("Dont have any {} key", c);
                                return;
                            },
                    }
                },
                _ => {
                        println!("Dont have any {} key", c);
                        return;
                    },
            }
        },
        _ => {
                println!("Dont have any {} key", c);
                return;
            },
    };
    
    println!("{}", output);

}
