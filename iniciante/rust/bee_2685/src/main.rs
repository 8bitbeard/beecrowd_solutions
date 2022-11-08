use std::io;

fn main() {
    loop {
        let mut angle = String::new();
        io::stdin()
            .read_line(&mut angle)
            .expect("Failed to read line");
        let angle: u32 = match angle.trim() {
            "" => break,
            x => x.parse::<u32>().unwrap() % 360,
        };
        println!(
            "{}",
            match angle {
                (0..=89) => "Bom Dia!!",
                (90..=179) => "Boa Tarde!!",
                (180..=269) => "Boa Noite!!",
                _ => "De Madrugada!!",
            }
        );
    }
}
