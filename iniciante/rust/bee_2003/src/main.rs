use std::io;

const MEET_TIME: i32 = 480;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read the line!");
        let input = input.trim();
        if input.is_empty() { break };

        let wakeup_time = convert_to_minutes(input);

        let max_delay_time: i32 = wakeup_time + 60 - MEET_TIME;

        let output: i32 = if max_delay_time > 0 {
            max_delay_time
        } else {
            0
        };

        println!("Atraso maximo: {}", output);
    }
}

fn convert_to_minutes(time: &str) -> i32 {
    let split: Vec<i32> = time.split(':').map(|x| x.parse::<i32>().unwrap()).collect();
    split[0] * 60 + split[1]
}
