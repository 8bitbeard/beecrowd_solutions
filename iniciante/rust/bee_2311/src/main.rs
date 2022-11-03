use std::io;

fn main() {
    let mut amount = String::new();
    io::stdin()
        .read_line(&mut amount)
        .expect("Failed to read the amount");
    let amount: u32 = amount.trim().parse().unwrap();

    for _i in 0..amount {
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read the name");
        let name = name.trim();

        let mut start_grade = String::new();
        io::stdin()
            .read_line(&mut start_grade)
            .expect("Failed to read the name");
        let start_grade: f64 = start_grade.trim().parse().unwrap();

        let mut grades = String::new();
        io::stdin()
            .read_line(&mut grades)
            .expect("Failed to read the name");
        let mut grades: Vec<u32> = grades
            .trim()
            .split(' ')
            .map(|x| (x.parse::<f64>().unwrap() * 10.0) as u32)
            .collect();

        grades.sort();
        let valid_grades: Vec<f64> = grades[1..6].iter().map(|x| (*x as f64) / 10.0).collect();

        println!(
            "{} {:.2}",
            name,
            valid_grades.iter().sum::<f64>() * start_grade
        );
    }
}
