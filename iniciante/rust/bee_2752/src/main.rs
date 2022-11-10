fn main() {
    let s = String::from("AMO FAZER EXERCICIO NO URI");
    println!("<{}>", s);
    println!("<{:>30}>", s);
    println!("<{}>", &s[..20]);
    println!("<{}>", s);
    println!("<{:<30}>", s);
    println!("<{}>", s);
    println!("<{:>30}>", &s[..20]);
    println!("<{:<30}>", &s[..20]);
}
