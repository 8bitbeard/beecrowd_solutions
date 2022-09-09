use std::io;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let numbers: Vec<i32> = input
                                .trim()
                                .split(' ')
                                .map(|s| (s.parse::<f64>().unwrap() * 10.0) as i32)
                                .collect();
                            
                                
    let sides: Vec<i32> = sort_create_new(&numbers).into_iter().rev().collect();
    
    if sides[0] >= sides[1] + sides[2] {
        println!("NAO FORMA TRIANGULO");
        return;
    };
    
    if i32::pow(sides[0], 2) == i32::pow(sides[1], 2) + i32::pow(sides[2], 2) {
        println!("TRIANGULO RETANGULO");
    } else if i32::pow(sides[0], 2) > i32::pow(sides[1], 2) + i32::pow(sides[2], 2) {
        println!("TRIANGULO OBTUSANGULO");
    } else {
        println!("TRIANGULO ACUTANGULO");
    };
    
    let sides_set = hashset(&sides);
    
    if sides_set.len() == 1 {
        println!("TRIANGULO EQUILATERO");
    } else if sides_set.len() == 2 {
        println!("TRIANGULO ISOSCELES");
    };

}


fn hashset(data: &[i32]) -> HashSet<i32> {
    HashSet::from_iter(data.iter().cloned())
}

fn sort_create_new(vec: &[i32]) -> Vec<i32> {
    let mut newvec = vec.to_vec();
    newvec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    newvec
}
