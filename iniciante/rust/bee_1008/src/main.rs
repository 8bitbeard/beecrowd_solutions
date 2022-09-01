use std::io;

fn main() {
    let mut employee_number = String::new();
    let mut worked_hours = String::new();
    let mut value_per_hour = String::new();

    io::stdin().read_line(&mut employee_number).expect("Failed to read line!");
    let employee_number: u32 = employee_number.trim().parse().unwrap();

    io::stdin().read_line(&mut worked_hours).expect("Failed to read line!");
    let worked_hours: u32 = worked_hours.trim().parse().unwrap();

    io::stdin().read_line(&mut value_per_hour).expect("Failed to read line!");
    let value_per_hour: f64 = value_per_hour.trim().parse().unwrap();

    let salary: f64 = worked_hours as f64 * value_per_hour;

    println!("NUMBER = {}", employee_number);
    println!("SALARY = U$ {:.2}", salary);
}
