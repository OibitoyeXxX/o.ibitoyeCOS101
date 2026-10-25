use std::io::Write;

fn main() {
    // the header is a vector too
    let header = vec!["Student Name", "Matric. Number", "Department", "Level"];
    let names = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matrics = vec!["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let depts = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = vec!["300", "100", "200", "200", "100"];

    // 1. display on screen
    println!("=== PAU-SMIS ===");
    println!("{} | {} | {} | {}", header[0], header[1], header[2], header[3]);
    for i in 0..names.len() {
        println!("{} | {} | {} | {}", names[i], matrics[i], depts[i], levels[i]);
    }

    // 2. write to the file
    let mut file = std::fs::File::create("students.csv").expect("create failed");

    // header row
    file.write_all(header[0].as_bytes()).expect("write failed");
    file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(header[1].as_bytes()).expect("write failed");
    file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(header[2].as_bytes()).expect("write failed");
    file.write_all(",".as_bytes()).expect("write failed");
    file.write_all(header[3].as_bytes()).expect("write failed");
    file.write_all("\n".as_bytes()).expect("write failed");

    // one row per student
    for i in 0..names.len() {
        file.write_all(names[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(matrics[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(depts[i].as_bytes()).expect("write failed");
        file.write_all(",".as_bytes()).expect("write failed");
        file.write_all(levels[i].as_bytes()).expect("write failed");
        file.write_all("\n".as_bytes()).expect("write failed");
    }

    println!("\nStudent records saved to students.csv");
}