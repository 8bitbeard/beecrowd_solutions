use std::io;

fn main() {
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed");
        let n: usize = match n.trim().parse() {
            Ok(x) if x != 0 => x,
            _ => break,
        };

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed");
        let input: Vec<i32> = input
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut dir: bool = input[0] < input[1];
        let mut peaks: usize = 1;

        for i in 1..n {
            if (input[i - 1] < input[i]) != dir {
                peaks += 1;
                dir = !dir;
            }
        }

        println!("{}", peaks + 1)
    }
}
