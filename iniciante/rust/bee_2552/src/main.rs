use std::io;

fn main() {
    loop {
        let mut sides = String::new();
        io::stdin()
            .read_line(&mut sides)
            .expect("Failed to read the line");
        let sides: Vec<usize> = match sides.trim() {
            "" => break,
            x => x.split(' ').map(|x| x.parse::<usize>().unwrap()).collect(),
        };

        let mut matrix: Vec<Vec<u8>> = Vec::new();

        for _i in 0..sides[0] {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");
            let input: Vec<u8> = input
                .trim()
                .split(' ')
                .map(|x| x.parse::<u8>().unwrap())
                .collect();

            matrix.push(input);
        }

        for i in 0..sides[0] {
            let mut v: Vec<String> = Vec::new();
            for j in 0..sides[1] {
                v.push(check_sides(
                    i as i32,
                    j as i32,
                    sides[0] as i32 - 1,
                    sides[1] as i32 - 1,
                    &matrix,
                ))
            }
            println!("{}", v.join(""));
        }
    }
}

fn check_sides(l: i32, c: i32, h: i32, w: i32, matrix: &Vec<Vec<u8>>) -> String {
    let sides: Vec<[i32; 2]> = vec![[l - 1, c], [l, c + 1], [l + 1, c], [l, c - 1]];

    let valid_sides: Vec<[i32; 2]> = sides
        .into_iter()
        .filter(|x| x[0] >= 0 as i32 && x[0] <= h as i32 && x[1] >= 0 as i32 && x[1] <= w as i32)
        .collect::<Vec<_>>();

    let mut output = 0;
    for s in valid_sides {
        match matrix[s[0] as usize][s[1] as usize] {
            1 => output += 1,
            _ => (),
        }
    }

    match matrix[l as usize][c as usize] {
        1 => 9.to_string(),
        _ => output.to_string(),
    }
}
