use std::io;

fn main() {
    let mut v: Vec<String> = Vec::new();
    for _i in 0..3 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop();
        let input = input.to_string();

        v.push(input);
    }

    println!("{}{}{}", v[0], v[1], v[2]);
    println!("{}{}{}", v[1], v[2], v[0]);
    println!("{}{}{}", v[2], v[0], v[1]);
    println!("{}{}{}",
        get_slice(&v[0]),
        get_slice(&v[1]),
        get_slice(&v[2]),
    );
}

fn get_slice(input: &String) -> String {
    let s = input.to_owned();
    if s.chars().count() > 10 {
        return s[..10].to_string()
    } else {
        return s
    }
}
