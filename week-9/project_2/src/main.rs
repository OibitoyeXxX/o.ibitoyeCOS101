use std::io::Write;

fn main() {
    
    let header = vec!["Student Name", "Matric. Number", "Department", "Level"];
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matrics = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let depts = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = vec![300, 100, 200, 200, 100];

    
    println!("=== PAU-SMIS ===");
    println!("{} | {} | {} | {}", header[0], header[1], header[2], header[3]);
    for i in 0..names.len() {
        println!("{} | {} | {} | {}", names[i], matrics[i], depts[i], levels[i]);
    }

    
    let mut file = std::fs::File::create("students.csv").expect("create failed");
    let head_row = format!("{},{},{},{}\n", header[0], header[1], header[2], header[3]);
    file.write_all(head_row.as_bytes()).expect("write failed");
    for i in 0..names.len() {
        let row = format!("{},{},{},{}\n", names[i], matrics[i], depts[i], levels[i]);
        file.write_all(row.as_bytes()).expect("write failed");
    }

    println!("\nStudent records saved to students.csv");
}