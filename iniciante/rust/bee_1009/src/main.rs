use std::io;

fn main() {
    let mut employee_name = String::new();
    let mut fixed_salary = String::new();
    let mut total_sales = String::new();

    io::stdin().read_line(&mut employee_name).expect("Failed to read line!");
    let _employee_name = employee_name.trim();

    io::stdin().read_line(&mut fixed_salary).expect("Failed to read line!");
    let fixed_salary: f64 = fixed_salary.trim().parse().unwrap();

    io::stdin().read_line(&mut total_sales).expect("Failed to read line!");
    let total_sales: f64 = total_sales.trim().parse().unwrap();

    let final_salary: f64 = fixed_salary + (total_sales * 0.15);

    println!("TOTAL = R$ {:.2}", final_salary);
}
