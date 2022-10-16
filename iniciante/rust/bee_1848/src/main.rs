use std::io;

fn main() {
    'main_loop: loop {
        let mut total = 0;
        'get_input: loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line!");
            let input = input.trim();

            match input {
                "caw caw" => {
                    break 'get_input;
                },
                _ => {
                    let converted = input.replace("*", "1").replace("-", "0");
                    total += match u32::from_str_radix(&converted, 2) {
                        Ok(x) => x,
                        Err(_) => break 'main_loop,
                    }
                }
            }
        }
        println!("{}", total);
    }
}
