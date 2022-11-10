use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin().read_line(&mut tests).expect("Failed to read line");
    let tests: u32 = tests.trim().parse::<u32>().unwrap();
    
    for i in 0..tests {
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read line");
        let amount: u32 = amount.trim().parse::<u32>().unwrap();
        
        let mut elements: Vec<String> = Vec::new();
        for _i in 0..amount {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_string();
            
            elements.push(input);
        }
        
        let mut iter = String::new();
        io::stdin().read_line(&mut iter).expect("Failed to read line");
        let iter: usize = iter.trim().parse::<usize>().unwrap();
        
        for _i in 0..iter {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim().to_string();
            
            let mut dangerous: bool = false;
            for e in elements.iter() {
                match input.find(e) {
                    Some(_) => {
                        let s = input.split(e).collect::<Vec<&str>>();
                        let s = s.into_iter().filter(|x| !x.is_empty()).collect::<Vec<&str>>();
                        match s.into_iter().any(|x| {
                            let chr = x.chars().next().unwrap();
                            chr.is_numeric() || !chr.is_uppercase()
                        }) {
                            false => {
                                dangerous = true;
                                break;
                            },
                            true => ()
                        }
                    },
                    None => continue
                }
            }
            
            println!("{}", if dangerous { "Abortar" } else { "Prossiga" });
        }
        if i < tests - 1 {
            println!("");
        }
    }
}
