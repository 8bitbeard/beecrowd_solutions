use std::io;

fn main() {
    loop {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read the line");
        let data = data.trim();
        if data.is_empty() {
            break;
        }
        let data: Vec<u32> = data
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut analogimon: (i32, i32) = (0, 0);
        let mut trainer: (i32, i32) = (0, 0);

        for i in 0..data[0] {
            let mut line = String::new();
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read the line");
            let line: Vec<i32> = line
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            if line.contains(&1) {
                trainer = (i as i32, line.iter().position(|&r| r == 1).unwrap() as i32);
            }

            if line.contains(&2) {
                analogimon = (i as i32, line.iter().position(|&r| r == 2).unwrap() as i32);
            }
        }

        println!(
            "{}",
            (analogimon.0 - trainer.0).abs() + (analogimon.1 - trainer.1).abs()
        );
    }
}
