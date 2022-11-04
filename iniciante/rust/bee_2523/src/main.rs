use std::io;

fn main() {
    loop {
        let mut letters = String::new();
        io::stdin()
            .read_line(&mut letters)
            .expect("Failed to read the line");
        let letters = letters.trim();
        if letters.is_empty() {
            break;
        }
        let letters: Vec<char> = letters.chars().collect();

        let mut times_on = String::new();
        io::stdin()
            .read_line(&mut times_on)
            .expect("Failed to read the line");
        let _times_on: u32 = times_on.trim().parse().unwrap();

        let mut lighs_on = String::new();
        io::stdin()
            .read_line(&mut lighs_on)
            .expect("Failed to read the line");
        let lighs_on: Vec<u32> = lighs_on
            .trim()
            .split(' ')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let mut output: Vec<char> = Vec::new();
        for l in lighs_on {
            output.push(letters[l as usize - 1])
        }
        let phrase: Vec<String> = output.iter().map(|x| x.to_string()).collect();

        println!("{}", phrase.join(""))
    }
}
