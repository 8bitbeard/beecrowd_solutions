use std::io;

fn main() {
    let mut balls = String::new();
    io::stdin()
        .read_line(&mut balls)
        .expect("Failed to read the balls amount");
    let balls: i32 = balls.trim().parse::<i32>().unwrap();

    let mut branches = String::new();
    io::stdin()
        .read_line(&mut branches)
        .expect("Failed to read the branches amount");
    let branches: i32 = branches.trim().parse::<i32>().unwrap();

    let output: i32 = balls - branches / 2;

    match output >= 0 {
        true => println!("Amelia tem todas bolinhas!"),
        false => println!("Faltam {} bolinha(s)", output.abs()),
    }
}
