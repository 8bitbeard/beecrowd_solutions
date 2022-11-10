fn main() {
    for i in 0..26 {
        let value: u8 = 97 + i as u8;
        println!("{} e {}", value, value as char);
    }
}
