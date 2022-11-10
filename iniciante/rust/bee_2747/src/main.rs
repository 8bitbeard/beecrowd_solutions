const WIDTH: usize = 39;
const HEIGHT: usize = 5;

fn main() {
    print_line(String::from("-"), WIDTH);
    for _i in 0..HEIGHT {
        print_sides(String::from("|"), WIDTH);
    }
    print_line(String::from("-"), WIDTH);
}

fn print_line(ch: String, size: usize) {
    let mut line = String::new();
    line.push_str(&ch.repeat(size));

    println!("{}", line);
}

fn print_sides(ch: String, size: usize) {
    let mut line = String::new();
    line.push_str(&ch);
    line.push_str(&" ".repeat(size - 2));
    line.push_str(&ch);

    println!("{}", line);
}
