use std::io;

fn main() {
    // two vectors lined up by position: names[i] has years[i]
    let mut names: Vec<String> = Vec::new();
    let mut years: Vec<u32> = Vec::new();

    let mut count_input = String::new();
    println!("How many candidates were interviewed? ");
    io::stdin().read_line(&mut count_input).expect("Failed to read input");
    let count: i32 = count_input.trim().parse().expect("Invalid input");

    for n in 0..count {
        let mut name_input = String::new();
        println!("Candidate {} name: ", n + 1);
        io::stdin().read_line(&mut name_input).expect("Failed to read input");
        names.push(name_input.trim().to_string());

        let mut years_input = String::new();
        println!("Years of programming experience: ");
        io::stdin().read_line(&mut years_input).expect("Failed to read input");
        let y: u32 = years_input.trim().parse().expect("Invalid input");
        years.push(y);
    }

    if years.len() == 0 {
        println!("No candidates to compare.");
    } else {
        // best = position of the highest years seen so far
        let mut best = 0;
        for i in 0..years.len() {
            if years[i] > years[best] {
                best = i;
            }
        }
        println!("Highest experience: {} years", years[best]);
        println!("Candidate(s) with {} years:", years[best]);
        for i in 0..years.len() {
            if years[i] == years[best] {
                println!("  {}", names[i]);
            }
        }
    }
}