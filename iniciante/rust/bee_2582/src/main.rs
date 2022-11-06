use std::collections::HashMap;
use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: usize = tests.trim().parse::<usize>().unwrap();

    let mut album: HashMap<u8, &str> = HashMap::new();
    album.insert(0, "PROXYCITY");
    album.insert(1, "P.Y.N.G.");
    album.insert(2, "DNSUEY!");
    album.insert(3, "SERVERS");
    album.insert(4, "HOST!");
    album.insert(5, "CRIPTONIZE");
    album.insert(6, "OFFLINE DAY");
    album.insert(7, "SALT");
    album.insert(8, "ANSWER!");
    album.insert(9, "RAR?");
    album.insert(10, "WIFI ANTENNAS");

    for _i in 0..tests {
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read the line");
        let numbers: Vec<u8> = numbers
            .trim()
            .split(' ')
            .map(|x| x.parse::<u8>().unwrap())
            .collect();

        let value = numbers.iter().sum::<u8>();

        println!("{}", match album.get(&value) {
            Some(x) => x,
            None => "Not found!"
        });
    }
}
