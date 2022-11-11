use std::io;

fn main() {
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read the line");
    let a: Vec<f32> = a
        .trim()
        .split(' ')
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read the line");
    let b: Vec<f64> = b
        .trim()
        .split(' ')
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    println!("A = {:.6}, B = {:.6}", a[0], a[1]);
    println!("C = {:.6}, D = {:.6}", b[0], b[1]);
    println!("A = {:.1}, B = {:.1}", a[0], a[1]);
    println!("C = {:.1}, D = {:.1}", b[0], b[1]);
    println!("A = {:.2}, B = {:.2}", a[0], a[1]);
    println!("C = {:.2}, D = {:.2}", b[0], b[1]);
    println!("A = {:.3}, B = {:.3}", a[0], a[1]);
    println!("C = {:.3}, D = {:.3}", b[0], b[1]);

    let f1 = format!("{:.3e}", a[0]);
    let mut split_1 = f1.split("e");
    let ta = (
        split_1.next().unwrap(),
        split_1.next().unwrap().parse::<u32>().unwrap(),
    );
    let f2 = format!("{:.3e}", a[1]);
    let mut split_2 = f2.split("e");
    let tb = (
        split_2.next().unwrap(),
        split_2.next().unwrap().parse::<u32>().unwrap(),
    );

    println!("A = {}E+{:02}, B = {}E+{:02}", ta.0, ta.1, tb.0, tb.1);

    let f3 = format!("{:.3e}", b[0]);
    let mut split_3 = f3.split("e");
    let tc = (
        split_3.next().unwrap(),
        split_3.next().unwrap().parse::<u32>().unwrap(),
    );
    let f4 = format!("{:.3e}", b[1]);
    let mut split_4 = f4.split("e");
    let td = (
        split_4.next().unwrap(),
        split_4.next().unwrap().parse::<u32>().unwrap(),
    );

    println!("C = {}E+{:02}, D = {}E+{:02}", tc.0, tc.1, td.0, td.1);

    println!("A = {:.0}, B = {:.0}", a[0], a[1]);
    println!("C = {:.0}, D = {:.0}", b[0], b[1]);
}
