use std::io;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let mut vector: Vec<i32> = input
                                .trim()
                                .split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect();
    
    let sorted = sort_create_new(&vector);
    
    for item in sorted {
        println!("{}", item);
    }
    
    println!("");
    
    for item in vector {
        println!("{}", item);
    }
}

fn sort_create_new(vec: &[i32]) -> Vec<i32> {
    let mut newvec = vec.to_vec();
    newvec.sort_by(|a, b| a.partial_cmp(b).unwrap()); 
    newvec
}
