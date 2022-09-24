fn main() {
    for i in (1..=9).step_by(2) {
        for j in (5..=7).rev() {
            println!("I={} J={}", i, j);
        }
    }
}
