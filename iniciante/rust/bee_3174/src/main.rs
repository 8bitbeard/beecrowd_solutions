use std::collections::HashMap;
use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: usize = amount.trim().parse::<usize>().unwrap();

    let mut hm: HashMap<String, u32> = HashMap::new();
    hm.insert(String::from("bonecos"), 8);
    hm.insert(String::from("arquitetos"), 4);
    hm.insert(String::from("musicos"), 6);
    hm.insert(String::from("desenhistas"), 12);

    let mut values: (u32, u32, u32, u32) = (0, 0, 0, 0);
    for _i in 0..amount {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input: Vec<&str> = input.trim().split(' ').collect::<Vec<&str>>();

        let (_name, group, hours): (String, &str, u32) = (
            input[0].to_string(),
            input[1],
            input[2].parse::<u32>().unwrap(),
        );

        match group {
            "bonecos" => values.0 += hours,
            "arquitetos" => values.1 += hours,
            "musicos" => values.2 += hours,
            "desenhistas" => values.3 += hours,
            _ => (),
        }
    }

    let output = values.0 / hm.get("bonecos").unwrap()
        + values.1 / hm.get("arquitetos").unwrap()
        + values.2 / hm.get("musicos").unwrap()
        + values.3 / hm.get("desenhistas").unwrap();

    println!("{}", output);
}
