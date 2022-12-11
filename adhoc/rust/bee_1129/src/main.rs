use std::io;

fn main() {
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed");
        let n: usize = match n.trim().parse() {
            Ok(x) if x != 0 => x,
            _ => break,
        };

        for _i in 0..n {
            let mut i = String::new();
            io::stdin().read_line(&mut i).expect("Failed");
            let i: Vec<bool> = i
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap() <= 127)
                .collect();

            let marks = i.iter().filter(|x| *x == &true).count();
            println!(
                "{}",
                match marks {
                    1 => {
                        let letters = ["A", "B", "C", "D", "E"];
                        let idx = i.iter().position(|x| *x == true).unwrap();
                        letters[idx]
                    }
                    _ => "*",
                }
            )
        }
    }
}
