use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line!");

    let base: Vec<f64> = vec![2.0, 3.0, 4.0, 1.0];
    let vector: Vec<&str> = input.trim().split(' ').collect();

    let mut student_grades: Vec<f64> = Vec::new();
    for (pos, e) in vector.iter().enumerate() {
        let grade: f64 = e.parse().unwrap();
        student_grades.push(base[pos] * grade);
    }

    let media: f64 = student_grades.iter().sum::<f64>() / 10.0;

    println!("Media: {:.1}", media);

    if media < 5.0 {
        println!("Aluno reprovado.");
    } else if media < 7.0 {
        println!("Aluno em exame.");
            let mut input_two = String::new();

            io::stdin()
                .read_line(&mut input_two)
                .expect("failed to read line!");

            let input_two: f64 = input_two.trim().parse().unwrap();

            println!("Nota do exame: {:.1}", input_two);

            let media_final: f64 = (media +  input_two) / 2.0;

            if media_final < 5.0 {
                println!("Aluno reprovado.");
            }

            println!("Aluno aprovado.");
            println!("Media final: {:.1}", media_final);

    } else {
        println!("Aluno aprovado.");
    }
}
