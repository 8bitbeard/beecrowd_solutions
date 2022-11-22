use std::io;

fn main() {
    let mut moves = String::new();
    io::stdin()
        .read_line(&mut moves)
        .expect("Failed to read the moves");
    let moves: usize = moves.trim().parse::<usize>().unwrap();

    let mut start = String::new();
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read the moves");
    let mut start: Vec<u32> = match start.trim() {
        "A" => vec![1, 0, 0],
        "B" => vec![0, 1, 0],
        _ => vec![0, 0, 1],
    };

    for _i in 0..moves {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        match input.trim().parse::<u32>().unwrap() {
            1 => {
                let buff = start[0];
                start[0] = start[1];
                start[1] = buff;
            }
            2 => {
                let buff = start[1];
                start[1] = start[2];
                start[2] = buff;
            }
            _ => {
                let buff = start[0];
                start[0] = start[2];
                start[2] = buff;
            }
        };
    }

    println!(
        "{}",
        match start.iter().position(|&r| r == 1).unwrap() {
            0 => "A",
            1 => "B",
            _ => "C",
        }
    )
}
