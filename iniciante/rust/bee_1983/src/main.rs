use std::io;

struct Student {
    number: u32,
    grade: u32,
}

fn main() {
    let mut student_amount = String::new();
    io::stdin()
        .read_line(&mut student_amount)
        .expect("Failed to read the line!");
    let student_amount: usize = student_amount.trim().parse().unwrap();

    let mut students: Vec<Student> = Vec::new();

    for _s in 1..=student_amount {
        let mut student_data = String::new();
        io::stdin()
            .read_line(&mut student_data)
            .expect("Failed to read the line!");
        let student_data: Vec<f64> = student_data
            .trim()
            .split(' ')
            .map(|x| x.parse::<f64>().unwrap())
            .collect();

        students.push(Student {
            number: student_data[0] as u32,
            grade: (student_data[1] * 10.0) as u32,
        });
    }

    students.sort_by_key(|s| s.grade);

    let higher_grade_student = students.last().unwrap();

    if higher_grade_student.grade >= 80 {
        println!("{}", higher_grade_student.number);
    } else {
        println!("Minimum note not reached");
    }
}
