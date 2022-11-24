use std::io;

fn main() {
    let mut print = false;
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read number a");

        match (line.contains("<body>"), line.contains("</body>"), print) {
            (true, false, false) => print = true,
            (false, true, true) => print = false,
            (false, false, true) => {
                line.pop();
                println!("{}", line);
            }
            _ if line.trim() == "" => break,
            _ => continue,
        }
    }
}
