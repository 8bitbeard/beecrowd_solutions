use std::io;

const SECONDS_IN_DAY: f64 = 86400.0;

fn main() {
    loop {
        let mut angle = String::new();
        io::stdin()
            .read_line(&mut angle)
            .expect("Failed to read line");
        let angle: f64 = match angle.trim() {
            "" => break,
            x => x.parse::<f64>().unwrap() % 360.0,
        };

        let seconds = (angle * SECONDS_IN_DAY) / 360.0;
        let seconds = (seconds + 6.0 * 3600.0) % SECONDS_IN_DAY;

        println!(
            "{}",
            if angle >= 0.0 && angle < 90.0 {
                "Bom Dia!!"
            } else if angle >= 90.0 && angle < 180.0 {
                "Boa Tarde!!"
            } else if angle >= 180.0 && angle < 270.0 {
                "Boa Noite!!"
            } else {
                "De Madrugada!!"
            }
        );
        println!("{}", convert_to_time(seconds as u32));
    }
}

fn convert_to_time(seconds: u32) -> String {
    let h = seconds / 3600;
    let m = (seconds - (h * 3600)) / 60;
    let s = seconds - (h * 3600) - (m * 60);

    format!("{:02}:{:02}:{:02}", h, m, s)
}
