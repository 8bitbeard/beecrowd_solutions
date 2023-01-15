use std::io;

fn main() {
    let mut size = String::new();
    io::stdin()
        .read_line(&mut size)
        .expect("Failed to read the distance");
    let size: usize = size
        .trim()
        .parse::<usize>()
        .unwrap();

    let mut boxes: Vec<Vec<u32>> = vec![Vec::new(); size];

    for _i in 0..size {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read the distance");
        let line: Vec<u32> = line
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        for j in 0..size {
            boxes[j].push(line[j])
        }
    }
    println!("{:?}", boxes)
}
