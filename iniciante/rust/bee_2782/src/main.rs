use std::io;

fn main() {
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read the size");
    let size: usize = size.trim().parse::<usize>().unwrap();

    let mut seq = String::new();
    io::stdin()
        .read_line(&mut seq)
        .expect("Failed to read the distance");
    let seq: Vec<i32> = seq
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut seq_counter: u32 = 0;
    if size > 1 {
        let mut buffer: i32 = seq[1] - seq[0];
        for i in 1..size {
            let diff = seq[i] - seq[i - 1];
            match diff {
                x if x == buffer => continue,
                _ => {
                    buffer = diff;
                    seq_counter += 1;
                }
            }
        }
    }
    println!("{}", seq_counter + 1);
}
