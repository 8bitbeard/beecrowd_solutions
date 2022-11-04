use std::io;

fn main() {
    loop {
        let mut amount = String::new();
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read the line");
        let amount: u32 = match amount.trim().parse() {
            Ok(x) => x,
            _ => break,
        };

        let mut expressions: Vec<Vec<i32>> = Vec::new();
        for _i in 0..amount {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");
            let input = input.trim();

            let space_idx = input.find(" ").unwrap();
            let eq_idx = input.find("=").unwrap();

            let v: Vec<i32> = vec![
                input[..space_idx].parse::<i32>().unwrap(),
                input[space_idx + 1..eq_idx].parse::<i32>().unwrap(),
                input[eq_idx + 1..].parse::<i32>().unwrap(),
            ];

            expressions.push(v);
        }

        let mut losers: Vec<String> = Vec::new();
        for _i in 0..amount {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read the line");
            let input = input.trim();

            let split = input.split(' ').collect::<Vec<&str>>();
            let nr = split[1].parse::<usize>().unwrap() - 1;

            let result: bool = match split[2] {
                "+" => expressions[nr][0] + expressions[nr][1] == expressions[nr][2],
                "-" => expressions[nr][0] - expressions[nr][1] == expressions[nr][2],
                "*" => expressions[nr][0] * expressions[nr][1] == expressions[nr][2],
                _ => {
                    expressions[nr][0] + expressions[nr][1] != expressions[nr][2]
                    && expressions[nr][0] - expressions[nr][1] != expressions[nr][2]
                    && expressions[nr][0] * expressions[nr][1] != expressions[nr][2]
                },
            };

            if !result {
                losers.push(split[0].to_owned())
            }
        }

        losers.sort();

        let output = match losers.len() {
            0 => "You Shall All Pass!".to_string(),
            x if x == amount as usize => "None Shall Pass!".to_string(),
            _ => losers.join(" "),
        };

        println!("{}", output);
    }
}
