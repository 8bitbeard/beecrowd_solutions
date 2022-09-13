use std::io;

fn main() {

    let mut v: Vec<String> = Vec::new();
    let mut counter: u8 = 0;

    while counter < 4 {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_string();

        v.push(input.clone());

        counter += 1;
    };

    let start_day: u32 = v[0].replace("Dia ", "").parse().unwrap();
    let end_day: u32 = v[2].replace("Dia ", "").parse().unwrap();

    let start_hour_vector = v[1].split(" : ").collect::<Vec<&str>>();
    let start_hour_vector = start_hour_vector.iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let end_hour_vector = v[3].split(" : ").collect::<Vec<&str>>();
    let end_hour_vector = end_hour_vector.iter().map(|&x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let seconds_vector: Vec<u32> = vec![3600, 60, 1];

    let mut start_hour_in_seconds = 0;
    for (pos, e) in start_hour_vector.iter().enumerate() {
        start_hour_in_seconds += seconds_vector[pos] * e;
    }

    let mut end_hour_in_seconds = 0;
    for (pos, e) in end_hour_vector.iter().enumerate() {
        end_hour_in_seconds += seconds_vector[pos] * e;
    }

    let total = if end_hour_in_seconds < start_hour_in_seconds {
        (24 * 3600 - start_hour_in_seconds) + end_hour_in_seconds
    } else {
        end_hour_in_seconds - start_hour_in_seconds
    };

    let days = if end_hour_in_seconds < start_hour_in_seconds {
        end_day - (start_day + 1)
    } else {
        end_day - start_day
    };

    let hours = total / 3600;
    let minutes = (total - hours * 3600) / 60;
    let seconds = total - hours * 3600 - minutes * 60;

    println!("{} dia(s)", days);
    println!("{} hora(s)", hours);
    println!("{} minuto(s)", minutes);
    println!("{} segundo(s)", seconds);
}
