use std::collections::HashMap;
use std::io;

fn main() {
    let mut table: HashMap<&str, u32> = HashMap::new();
    table.insert("suco de laranja", 120);
    table.insert("morango fresco", 85);
    table.insert("mamao", 85);
    table.insert("goiaba vermelha", 70);
    table.insert("manga", 56);
    table.insert("laranja", 50);
    table.insert("brocolis", 34);

    loop {
        let mut food_count = String::new();
        io::stdin()
            .read_line(&mut food_count)
            .expect("Failed to read the line");
        let food_count: u32 = match food_count.trim().parse() {
            Ok(x) if x != 0 => x,
            _ => break,
        };

        let mut total: u32 = 0;
        for _i in 0..food_count {
            let mut food_data = String::new();
            io::stdin()
                .read_line(&mut food_data)
                .expect("Failed to read the line");
            let food_data = food_data.trim();

            let sp_idx: usize = food_data.find(" ").unwrap();

            let (amount, name) = (
                &food_data[..sp_idx].parse::<u32>().unwrap(),
                &food_data[sp_idx + 1..],
            );
            total += amount * table.get(name).unwrap();
        }

        match total {
            (0..=109) => println!("Mais {} mg", 110 - total),
            (110..=130) => println!("{} mg", total),
            _ => println!("Menos {} mg", total - 130),
        }
    }
}
