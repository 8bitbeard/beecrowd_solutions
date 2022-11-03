use std::io;

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the tests");
    let tests: u32 = tests.trim().parse().unwrap();

    for _i in 0..tests {
        let mut bonus = String::new();
        io::stdin()
            .read_line(&mut bonus)
            .expect("Failed to read the bonus");
        let bonus: f64 = bonus.trim().parse().unwrap();

        let mut player_one_stats = String::new();
        io::stdin()
            .read_line(&mut player_one_stats)
            .expect("Failed to read the player_one_stats");
        let player_one_stats: Vec<f64> = player_one_stats
            .trim()
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let mut player_two_stats = String::new();
        io::stdin()
            .read_line(&mut player_two_stats)
            .expect("Failed to read the player_one_two");
        let player_two_stats: Vec<f64> = player_two_stats
            .trim()
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        let player_one_attack = (player_one_stats[0] + player_one_stats[1]) / 2.0
            + if player_one_stats[2] as u32 % 2 == 0 {
                bonus
            } else {
                0.0
            };
        let player_two_attack = (player_two_stats[0] + player_two_stats[1]) / 2.0
            + if player_two_stats[2] as u32 % 2 == 0 {
                bonus
            } else {
                0.0
            };

        if player_one_attack > player_two_attack {
            println!("Dabriel");
        } else if player_one_attack < player_two_attack {
            println!("Guarte");
        } else {
            println!("Empate");
        }
    }
}
