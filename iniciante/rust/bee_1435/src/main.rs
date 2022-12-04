use std::io;

fn main() {

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        let size = match input.trim().parse::<i32>() {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        let mut matrix: Vec<Vec<i32>> = Vec::new();

        let template = build_line(&size);

        for _i in 1..=size {
            let line = build_line(&size);
            matrix.push(line);
        }

        for (pos, e) in template.iter().enumerate() {
            let joined = matrix[pos]
                .iter()
                .map(|x| format!("{:>3}", if x > e { e } else { x }))
                .collect::<Vec<String>>()
                .join(" ");
            println!("{}", joined);
        }
         println!("");
    }
}

fn build_line(size: &i32) -> Vec<i32> {
    let mut template: Vec<i32> = (0..=*size)
        .map(|x| i32::abs((*size + 1) / 2 - x) )
        .filter(|x| *x != 0)
        .collect();
    template.rotate_right(*size as usize / 2);

    template
}
