use std::io;
use std::collections::HashSet;

fn main() {
    let mut star_count = String::new();
    io::stdin()
        .read_line(&mut star_count)
        .expect("Failed to read line");
    let star_count: usize = star_count.trim().parse().unwrap();

    let mut lamb_count = String::new();
    io::stdin()
        .read_line(&mut lamb_count)
        .expect("Failed to read line");
    let mut lamb_count: Vec<u64> = lamb_count.trim().split(' ').map(|x| x.parse::<u64>().unwrap()).collect();

    let mut current_position: i32 = 0;
    let mut stars_attacked = HashSet::new();

    loop {
        if current_position >= 0 && current_position < star_count as i32 {
            let current_lamb_amount = lamb_count[current_position as usize];
            if current_lamb_amount == 0 { break };
            lamb_count[current_position as usize] -= 1;
            stars_attacked.insert(current_position);
            if current_lamb_amount % 2 == 0 { current_position -= 1 } else { current_position += 1 }
        } else {
            break;
        }
    }

    println!("{} {}", stars_attacked.len(), lamb_count.iter().sum::<u64>() )
}
