use std::io;

struct Ship {
    x: f64,
    y: f64,
    z: f64,
    signal: char,
}

impl Ship {
    fn distance(&self, other: &Ship) -> f64 {
        return ((self.x - other.x).powf(2.0)
            + (self.y - other.y).powf(2.0)
            + (self.z - other.z).powf(2.0))
        .sqrt();
    }

    fn signal_power(&self, other: &Ship) -> char {
        let d = self.distance(other);
        match d {
            x if x <= 20.0 => 'A',
            x if x <= 50.0 => 'M',
            _ => 'B',
        }
    }

    fn set_best_signal(&mut self, other: &mut Ship) {
        let signal_power = self.signal_power(other);
        self.signal = match (self.signal, signal_power) {
            ('B', 'A') | ('B', 'M') | ('M', 'A') => signal_power,
            _ => self.signal,
        };

        other.signal = match (other.signal, signal_power) {
            ('B', 'A') | ('B', 'M') | ('M', 'A') => signal_power,
            _ => other.signal,
        }
    }
}

fn build_ship(coords: String) -> Ship {
    let v: Vec<f64> = coords
        .split(' ')
        .map(|x| x.parse::<f64>().unwrap())
        .collect();
    Ship {
        x: v[0],
        y: v[1],
        z: v[2],
        signal: 'B',
    }
}

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read line");
    let amount: usize = amount.trim().parse().unwrap();

    let mut ships: Vec<Ship> = Vec::new();
    for _i in 0..amount {
        let mut coords = String::new();
        io::stdin()
            .read_line(&mut coords)
            .expect("Failed to read input");
        let mut ship = build_ship(coords.trim().to_string());

        for j in 0..ships.len() {
            ship.set_best_signal(&mut ships[j])
        }

        ships.push(ship);
    }

    for s in ships.iter() {
        println!("{}", s.signal);
    }
}
