use std::io;


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");
        
    let numbers: Vec<i32> = input
                                .trim()
                                .split(' ')
                                .map(|s| s.parse().unwrap())
                                .collect();
                                
    let sorted_numbers = sort_create_new(&numbers);
    
    if sorted_numbers[1] % sorted_numbers[0] == 0 {
        println!("Sao Multiplos");
        return;
    }
    println!("Nao sao Multiplos");
}

fn sort_create_new(vec: &[i32]) -> Vec<i32> {
    let mut newvec = vec.to_vec();
    newvec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    newvec
}
