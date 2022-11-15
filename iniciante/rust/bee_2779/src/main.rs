use std::collections::HashSet;
use std::io;

fn main() {
    let mut total = String::new();
    io::stdin()
        .read_line(&mut total)
        .expect("Failed to read the line!");
    let total = total.trim().parse::<usize>().unwrap();

    let mut iter = String::new();
    io::stdin()
        .read_line(&mut iter)
        .expect("Failed to read the line!");
    let iter = iter.trim().parse::<usize>().unwrap();

    let mut items: HashSet<u32> = HashSet::new();
    for _i in 0..iter {
        let mut item = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("Failed to read the line!");
        let item = item.trim().parse::<u32>().unwrap();

        items.insert(item);
    }

    println!("{}", total - items.len());
}
