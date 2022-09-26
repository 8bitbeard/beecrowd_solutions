use std::io;

fn main() {
    'outer: loop{
        let mut grades: Vec<f64> = Vec::new();

        while grades.len() < 2 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line!");
    
            let input: f64 = input.trim().parse().unwrap();
    
            if input >= 0.0 && input <= 10.0 {
                grades.push(input);
            } else {
                println!("nota invalida");
            }
        }

        println!("media = {:.2}", (grades[0] + grades[1]) / 2.0);
        
        'question: loop {
            println!("novo calculo (1-sim 2-nao)");
            
            let mut answer = String::new();
            io::stdin().read_line(&mut answer).expect("Failed to read line!");
            let answer = answer.trim();
            
            match answer {
                "1" => break 'question,
                "2" => break 'outer,
                _ => continue
            }
        }
    }
}
