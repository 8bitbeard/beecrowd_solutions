use std::io;

fn main() {
    let mut values = String::new();
    io::stdin()
        .read_line(&mut values)
        .expect("Failed to read the amount");
    let values: Vec<i32> = values
        .trim()
        .split(' ')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let (f, s, g, u, d): (i32, i32, i32, i32, i32) =
        (values[0], values[1], values[2], values[3], values[4]);

    if (u == d && (g - s).abs() % u != 0) || (g > s && u == 0) || (g < s && d == 0) {
        println!("use the stairs");
        return;
    }

    match (s - g).abs() % (u - d).abs() == 0 {
        true => {
            let mut moves = 0;
            let mut current = s;
            while current != g {
                if current < g && current + u <= f {
                    current += u
                } else {
                    current -= d
                }
                moves += 1;
            }
            println!("{}", moves);
            return;
        }
        false => {
            println!("use the stairs");
            return;
        }
    }
}
