use std::cmp;
use std::io;

struct Enemy {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

struct MagicAttack {
    x: i32,
    y: i32,
    damage: i32,
    radius: i32,
}

impl MagicAttack {
    fn hits(&self, enemy: &Enemy) -> bool {
        let xn = cmp::max(enemy.x, cmp::min(self.x, enemy.x + enemy.w));
        let yn = cmp::max(enemy.y, cmp::min(self.y, enemy.y + enemy.h));

        let dx = (xn - self.x) as f64;
        let dy = (yn - self.y) as f64;

        dx.powf(2.0) + dy.powf(2.0) <= (self.radius as f64).powf(2.0)
    }
}

impl MagicAttack {
    fn fire(x: i32, y: i32, level: i32) -> Self {
        Self {
            x,
            y,
            damage: 200,
            radius: match level {
                1 => 20,
                2 => 30,
                _ => 50,
            },
        }
    }

    fn water(x: i32, y: i32, level: i32) -> Self {
        Self {
            x,
            y,
            damage: 300,
            radius: match level {
                1 => 10,
                2 => 25,
                _ => 40,
            },
        }
    }

    fn earth(x: i32, y: i32, level: i32) -> Self {
        Self {
            x,
            y,
            damage: 400,
            radius: match level {
                1 => 25,
                2 => 55,
                _ => 70,
            },
        }
    }

    fn air(x: i32, y: i32, level: i32) -> Self {
        Self {
            x,
            y,
            damage: 100,
            radius: match level {
                1 => 18,
                2 => 38,
                _ => 60,
            },
        }
    }
}

fn main() {
    let mut tests = String::new();
    io::stdin()
        .read_line(&mut tests)
        .expect("Failed to read the line");
    let tests: usize = tests.trim().parse::<usize>().unwrap();

    for _i in 1..=tests {
        let mut enemy_data = String::new();
        io::stdin()
            .read_line(&mut enemy_data)
            .expect("Failed to read strategy");
        let enemy_data: Vec<i32> = enemy_data
            .trim()
            .split(' ')
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let (w, h, x0, y0): (i32, i32, i32, i32) =
            (enemy_data[0], enemy_data[1], enemy_data[2], enemy_data[3]);
        let enemy = Enemy {
            x: x0,
            y: y0,
            w,
            h,
        };

        let mut attack_data = String::new();
        io::stdin()
            .read_line(&mut attack_data)
            .expect("Failed to read strategy");
        let attack_data: Vec<&str> = attack_data.trim().split(' ').collect();

        let data: Vec<i32> = attack_data[1..]
            .iter()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let (level, cx, cy): (i32, i32, i32) = (data[0], data[1], data[2]);

        let magic_attack = match attack_data[0] {
            "fire" => MagicAttack::fire(cx, cy, level),
            "water" => MagicAttack::water(cx, cy, level),
            "earth" => MagicAttack::earth(cx, cy, level),
            "air" => MagicAttack::air(cx, cy, level),
            _ => break,
        };

        let damage: i32 = match magic_attack.hits(&enemy) {
            true => magic_attack.damage,
            false => 0,
        };

        println!("{}", damage);
    }
}
