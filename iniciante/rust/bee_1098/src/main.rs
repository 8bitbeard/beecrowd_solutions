fn main() {
    let mut modifier = 0.0;
    let mut counter = 0;

    while modifier < 2.0 {
        for j in 1..=3 {
            if counter == 0 {
                println!("I={:.0} J={:.0}", modifier, j as f64 + modifier);
            } else {
                println!("I={:.1} J={:.1}", modifier, j as f64 + modifier);
            }
        }
        modifier += 0.2;
        counter = (counter + 1) % 5;
    }
}
