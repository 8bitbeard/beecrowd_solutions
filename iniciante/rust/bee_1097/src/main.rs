fn main() {
    let mut modifier = 7;

    for i in (1..=9).step_by(2) {
        for j in (modifier-2..=modifier).rev() {
            println!("I={} J={}", i, j);
        }
        modifier += 2;
    }
}
