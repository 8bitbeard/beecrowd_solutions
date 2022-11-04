use std::io;

fn main() {
    loop {
        let mut data = String::new();
        io::stdin()
            .read_line(&mut data)
            .expect("Failed to read the line");

        let (n, q): (u32, u32) = match data.trim() {
            "" => break,
            x => {
                let space_idx = x.find(' ').unwrap();
                let (i, j) = (&x[..space_idx], &x[space_idx + 1..]);
                (i.parse::<u32>().unwrap(), j.parse::<u32>().unwrap())
            }
        };

        let mut grades: Vec<u32> = Vec::new();
        for _i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input: u32 = input.trim().parse::<u32>().unwrap();

            grades.push(input);
        }
        grades.sort_by(|a, b| b.cmp(a));

        for _i in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input: usize = input.trim().parse::<usize>().unwrap();

            println!("{}", &grades[input - 1])
        }

    }
}
