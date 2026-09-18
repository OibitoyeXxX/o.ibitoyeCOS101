use std::io;
fn main() {
    let mut experience_input = String::new();
    let mut age_input = String::new();

    println!("Are you an experienced employee? (yes or no) ");
    io::stdin().read_line(&mut experience_input).expect("Not a valid string");
    let experience = experience_input.trim().to_lowercase();

    let incentive: f64;

    if experience == "yes" {
        println!("Enter your age ->");
        io::stdin().read_line(&mut age_input).expect("Not a valid string");
        let age: i32 = age_input.trim().parse().expect("Not a valid number");

        if age >= 40 {
            incentive = 1_560_000.0;
        } else if age >= 30 && age <= 39 {
            incentive = 1_480_000.0;
        } else if age < 28 {
            incentive = 1_300_000.0;
        } else {
            incentive = 1_300_000.0; 
        }
    } else {
        incentive = 100_000.0; 
    }

    println!("Annual incentive : N {}", incentive);
}