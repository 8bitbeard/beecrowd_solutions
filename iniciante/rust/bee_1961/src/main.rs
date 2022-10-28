use std::io;

fn main() {
    let mut input_one = String::new();
    io::stdin()
        .read_line(&mut input_one)
        .expect("Failed to read line");
    let input_one: Vec<i32> = input_one
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let (jump, _pipes): (i32, i32) = (input_one[0], input_one[1]);

    let mut input_two = String::new();
    io::stdin()
        .read_line(&mut input_two)
        .expect("Failed to read line");
    let pipe_heights: Vec<i32> = input_two
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut current_position = pipe_heights[0];
    for height in &pipe_heights[1..] {
        if (*height - current_position).abs() > jump {
            println!("GAME OVER");
            return;
        }
        current_position = *height;
    }
    println!("YOU WIN")
}
