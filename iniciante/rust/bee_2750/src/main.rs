const WIDTH: usize = 39;

fn main() {
    print_outer(String::from("-"), WIDTH);
    print_inner(
        String::from("|"),
        String::from("decimal"),
        String::from("octal"),
        String::from("Hexadecimal"),
    );
    print_outer(String::from("-"), WIDTH);
    for i in 0..=15 {
        print_inner(
            String::from("|"),
            format!("{:>3}", i),
            format!("{:o}", i),
            format!("{:X}", i),
        );
    }
    print_outer(String::from("-"), WIDTH);
}

fn print_outer(ch: String, size: usize) {
    println!("{}", &ch.repeat(size));
}

fn print_inner(separator: String, col_a: String, col_b: String, col_c: String) {
    println!(
        "{}{:^11}{}{:^9}{}{:^15}{}",
        separator, col_a, separator, col_b, separator, col_c, separator
    );
}
