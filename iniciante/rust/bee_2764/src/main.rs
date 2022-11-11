use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    let input = input.trim();

    let v: Vec<&str> = input.split('/').collect();
    let (d, m, y) = (v[0], v[1], v[2]);

    println!("{}/{}/{}", m, d, y);
    println!("{}/{}/{}", y, m, d);
    println!("{}-{}-{}", d, m, y);
}
