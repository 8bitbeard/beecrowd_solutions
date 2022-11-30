use std::io;

fn main() {
    let mut can = String::new();
    io::stdin().read_line(&mut can).expect("Failed to read can");
    let can = can.trim();

    let mut must = String::new();
    io::stdin()
        .read_line(&mut must)
        .expect("Failed to read must");
    let must = must.trim();

    let can_count = can.chars().filter(|x| *x == 'a').count();
    let must_count = must.chars().filter(|x| *x == 'a').count();

    println!(
        "{}",
        match can_count >= must_count {
            true => "go",
            false => "no",
        }
    )
}
