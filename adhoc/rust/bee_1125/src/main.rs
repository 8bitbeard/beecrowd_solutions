use std::io;

#[derive(Debug)]
struct Pilot {
    number: u32,
    positions: Vec<u32>,
    points: u32,
}

impl Pilot {
    fn add_position(&mut self, position: u32) {
        self.positions.push(position);
    }

    fn calculate_points(&mut self, values: &Vec<u32>) {
        for p in self.positions.iter() {
            self.points += match values.get(*p as usize) {
                Some(x) => *x,
                None => 0,
            }
        }
    }

    fn reset_points(&mut self) {
        self.points = 0;
    }
}

fn main() {
    loop {
        let mut gp = String::new();
        io::stdin().read_line(&mut gp).expect("Failed");
        let gp: Vec<u32> = match gp.trim() {
            "0 0" => break,
            x => x.split(' ').map(|x| x.parse::<u32>().unwrap()).collect(),
        };

        let (g, p): (u32, u32) = (gp[0], gp[1]);

        let mut pilots: Vec<Pilot> = Vec::new();
        for i in 1..=p {
            pilots.push(Pilot {
                number: i as u32,
                positions: Vec::new(),
                points: 0,
            })
        }

        for _i in 0..g {
            let mut race = String::new();
            io::stdin().read_line(&mut race).expect("Failed");
            let race: Vec<u32> = race
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            for j in 0..race.len() {
                pilots[j].add_position(race[j])
            }
        }

        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("Failed");
        let s: usize = s.trim().parse().unwrap();

        for _i in 0..s {
            let mut system = String::new();
            io::stdin().read_line(&mut system).expect("Failed");
            let system: Vec<u32> = system
                .trim()
                .split(' ')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            for p in 0..pilots.len() {
                pilots[p].reset_points();
                pilots[p].calculate_points(&system);
            }

            pilots.sort_by_key(|p| p.points);

            let max = pilots.last().unwrap().points;
            let mut winners: Vec<u32> = pilots
                .iter()
                .filter(|x| x.points == max)
                .map(|x| x.number)
                .collect();
            winners.sort();

            let winners: Vec<String> = winners.into_iter().map(|x| x.to_string()).collect();

            println!("{}", winners.join(" "));
        }
    }
}
