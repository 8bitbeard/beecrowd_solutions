use std::io;

const MONTH_DAYS: [i32; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn main() {
    loop {
        let mut days = String::new();
        io::stdin()
            .read_line(&mut days)
            .expect("Failed to read the line!");
        let days = days.trim();
        let days: Vec<i32> = match days.is_empty() {
            true => break,
            false => days.split(' ').map(|x| x.parse::<i32>().unwrap()).collect(),
        };

        let (month, day): (usize, i32) = (days[0] as usize - 1, days[1]);

        let remaining_days: i32 = &MONTH_DAYS[month..12].iter().sum::<i32>() - day - 6;

        match remaining_days {
            0 => println!("E natal!"),
            1 => println!("E vespera de natal!"),
            (2..=360) => println!("Faltam {} dias para o natal!", remaining_days),
            _ => println!("Ja passou!"),
        }
    }
}
