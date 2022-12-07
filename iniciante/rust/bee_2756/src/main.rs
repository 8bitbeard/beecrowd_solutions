fn main() {
    for i in 1..=5 {
        build_line(i);
    }
    for i in (1..=4).rev() {
        build_line(i);
    }
}

fn build_line(idx: usize) {
    let size = idx * 2 - 1;
    let chars: Vec<char> = vec![' ', 'A', 'B', 'C', 'D', 'E'];
    let mut v: Vec<char> = vec![' '; size];
    v[0] = chars[idx];
    v.reverse();
    v[0] = chars[idx];

    println!("{:>width$}", v.iter().collect::<String>(), width = idx + 7);
}
