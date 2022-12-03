use std::io;

struct Nephew {
    idx: usize,
    age: u32,
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");
    let input: Vec<u32> = input
        .trim()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut v: Vec<Nephew> = Vec::new();
    for i in 0..input.len() {
        v.push(Nephew {
            idx: i,
            age: input[i],
        })
    }

    v.sort_by(|a, b| b.age.cmp(&a.age));

    println!(
        "{}",
        match v[1].idx {
            0 => "huguinho",
            1 => "zezinho",
            _ => "luisinho",
        }
    )
}
