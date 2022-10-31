use std::io;

fn main() {
    let mut counter: u32 = 1;

    loop {
        let mut sub = String::new();
        io::stdin()
            .read_line(&mut sub)
            .expect("Failed to read the line!");
        let sub = sub.trim();
        if sub.is_empty() {
            break;
        }

        let mut main = String::new();
        io::stdin()
            .read_line(&mut main)
            .expect("Failed to read the line!");
        let main = main.trim();
        if main.is_empty() {
            break;
        }

        let sub_chars: Vec<String> = sub.chars().into_iter().map(|x| x.to_string()).collect();
        let main_chars: Vec<String> = main.chars().into_iter().map(|x| x.to_string()).collect();

        let mut indexes: Vec<u32> = Vec::new();

        for i in 0..=(main_chars.len() - sub_chars.len()) {
            let slice = &main_chars[i..i + sub_chars.len()].to_vec();
            if &sub_chars == slice {
                indexes.push(i as u32 + 1);
            }
        }

        println!("Caso #{}:", counter);

        if indexes.len() == 0 {
            println!("Nao existe subsequencia");
        } else {
            println!("Qtd.Subsequencias: {}", indexes.len());
            println!("Pos: {}", indexes.last().unwrap());
        }
        println!("");
        counter += 1;
    }
}
