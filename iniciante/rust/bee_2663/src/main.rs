use std::io;

fn main() {
    let mut competitors = String::new();
    io::stdin()
        .read_line(&mut competitors)
        .expect("Failed to read line");
    let competitors = competitors.trim().parse::<usize>().unwrap();

    let mut spots = String::new();
    io::stdin()
        .read_line(&mut spots)
        .expect("Failed to read line");
    let spots = spots.trim().parse::<usize>().unwrap();

    let mut grades: Vec<u32> = Vec::new();
    for _i in 0..competitors {
        let mut grade = String::new();
        io::stdin()
            .read_line(&mut grade)
            .expect("Failed to read line");
        let grade: u32 = grade.trim().parse::<u32>().unwrap();

        grades.push(grade);
    }

    grades.sort();
    grades.reverse();

    let mut approved: Vec<u32> = Vec::new();
    for i in grades.iter() {
        if approved.len() < spots || approved.contains(i) {
            approved.push(*i);
        }
    }

    println!("{}", approved.len());
}
