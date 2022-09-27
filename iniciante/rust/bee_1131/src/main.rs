use std::io;

fn main() {
    let mut scores: Vec<Vec<u32>> = Vec::new();
    
    'outer: loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line!");

        let result: Vec<&str> = input.trim().split(' ').collect();
        let result: Vec<u32> = result.iter().map(|&x| x.parse::<u32>().unwrap()).collect();
        
        scores.push(result);
        
        'question: loop {
            println!("Novo grenal (1-sim 2-nao)");
            
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
    
    println!("{} grenais", scores.len());
    
    let mut statistics: [u32; 3] = [0, 0, 0];
    
    for score in scores {
        if score[0] > score[1] {
            statistics[0] += 1;
        } else if score[0] < score[1] {
            statistics[1] += 1;
        } else {
            statistics[2] += 1;
        }
    }
    
    println!("Inter:{}", statistics[0]);
    println!("Gremio:{}", statistics[1]);
    println!("Empates:{}", statistics[2]);
    
    if statistics[0] > statistics[1] {
        println!("Inter venceu mais");
    } else if statistics[0] < statistics[1] {
        println!("Gremio venceu mais");
    } else {
        println!("Não houve vencedor");
    }
}
