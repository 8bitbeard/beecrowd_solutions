use std::io;

fn main() {
    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read n");
        let n: usize = match n.trim().parse() {
            Ok(x) if x > 0 => x,
            _ => break,
        };

        let mut point = String::new();
        io::stdin()
            .read_line(&mut point)
            .expect("Failed to read the point");
        let point: Vec<i32> = point
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let point: (i32, i32) = (point[0], point[1]);

        for _i in 0..n {
            let mut p = String::new();
            io::stdin()
                .read_line(&mut p)
                .expect("Failed to read the point");
            let p: Vec<i32> = p
                .trim()
                .split(' ')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            let p: (i32, i32) = (p[0], p[1]);

            println!("{}", if p.0 > point.0 && p.1 > point.1 {
                "NE"
            } else if p.0 > point.0 && p.1 < point.1 {
                "SE"
            } else if p.0 < point.0 && p.1 > point.1 {
                "NO"
            } else if p.0 < point.0 && p.1 < point.1 {
                "SO"
            } else {
                "divisa"
            })
        }
    }
}
