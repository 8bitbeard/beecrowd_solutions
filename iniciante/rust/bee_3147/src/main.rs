use std::io;

fn main() {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("Failed to read input");
    let i: Vec<u32> = i
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let (h, e, a, o, w, x): (u32, u32, u32, u32, u32, u32) = (i[0], i[1], i[2], i[3], i[4], i[5]);

    let good = h + e + a;
    let bad = o + w;

    println!(
        "{}",
        match (good >= bad, good + x >= bad) {
            (true, true) => "Middle-earth is safe.",
            (false, true) => "Middle-earth is safe.",
            _ => "Sauron has returned.",
        }
    );
}
