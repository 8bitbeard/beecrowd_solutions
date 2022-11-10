fn main() {
    let a: f64 = 234.345;
    let b: f64 = 45.698;

    println!("{:.06} - {:.06}", a, b);
    println!("{:.0} - {:.0}", a, b);
    println!("{:.01} - {:.01}", a, b);
    println!("{:.02} - {:.02}", a, b);
    println!("{:.03} - {:.03}", a, b);

    let f = format!("{:.6e}", a);
    let mut split_a = f.split("e");
    let ta = (
        split_a.next().unwrap(),
        split_a.next().unwrap().parse::<u32>().unwrap(),
    );
    let f = format!("{:.6e}", b);
    let mut split_b = f.split("e");
    let tb = (
        split_b.next().unwrap(),
        split_b.next().unwrap().parse::<u32>().unwrap(),
    );

    println!("{}e+{:02} - {}e+{:02}", ta.0, ta.1, tb.0, tb.1);

    let f = format!("{:.6E}", a);
    let mut split_a = f.split("E");
    let ta = (
        split_a.next().unwrap(),
        split_a.next().unwrap().parse::<u32>().unwrap(),
    );
    let f = format!("{:.6E}", b);
    let mut split_b = f.split("E");
    let tb = (
        split_b.next().unwrap(),
        split_b.next().unwrap().parse::<u32>().unwrap(),
    );

    println!("{}E+{:02} - {}E+{:02}", ta.0, ta.1, tb.0, tb.1);
    println!("{} - {}", a, b);
    println!("{} - {}", a, b);
}
