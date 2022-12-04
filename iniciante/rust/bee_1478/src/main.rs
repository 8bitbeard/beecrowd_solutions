use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        let size = match input.trim().parse::<i32>() {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        for i in 0..size {
            let line = build_line(&size, i);
            let joined = line
                .iter()
                .map(|x| format!("{:>3}", x))
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", joined);
        }
        println!("");
    }
}

fn build_line(size: &i32, idx: i32) -> Vec<i32> {
    let mut line: Vec<i32> = Vec::new();
    let mut c: i32 = idx;
    for _i in 0..*size {
        line.push(c.abs());
        c = c - 1;
    }

    line.iter().map(|x| x + 1).collect()
}
