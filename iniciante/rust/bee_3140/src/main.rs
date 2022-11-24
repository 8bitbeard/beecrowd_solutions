use std::io;

fn main() {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read number a");
        match line.trim() {
            "<body>" => break,
            _ => continue,
        }
    }

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read number a");
        line.pop();

        match line.contains("</body>") {
            true => break,
            false => println!("{}", line),
        }
    }

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read number a");
        match line.trim() {
            "" => break,
            _ => continue,
        }
    }
}
