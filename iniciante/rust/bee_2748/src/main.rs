const WIDTH: usize = 39;

fn main() {
    print_outer(String::from("-"), WIDTH);
    print_inner(String::from("|"), String::from("Roberto"), 10, WIDTH);
    print_inner(String::from("|"), String::from(" "), 10, WIDTH);
    print_inner(String::from("|"), String::from("5786"), 10, WIDTH);
    print_inner(String::from("|"), String::from(" "), 10, WIDTH);
    print_inner(String::from("|"), String::from("UNIFEI"), 10, WIDTH);
    print_outer(String::from("-"), WIDTH);
}

fn print_outer(ch: String, size: usize) {
    let mut line = String::new();
    line.push_str(&ch.repeat(size));

    println!("{}", line);
}

fn print_inner(side: String, text: String, idx: usize, size: usize) {
    let mut line = String::new();
    line.push_str(&side);
    line.push_str(&" ".repeat(idx - 2));
    line.push_str(&text);
    line.push_str(&" ".repeat(size - text.chars().count() as usize - idx));
    line.push_str(&side);

    println!("{}", line);
}
