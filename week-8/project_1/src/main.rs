use std::io;

fn main() {
    // the levels (same position in every vector = same level)
    let levels = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];

    println!("=== APS LEVEL CHECKER ===");
    println!("1 - Office Admin");
    println!("2 - Academic");
    println!("3 - Lawyer");
    println!("4 - Teacher");

    let mut career_input = String::new();
    println!("Choose a career track (1-4): ");
    io::stdin().read_line(&mut career_input).expect("Failed to read input");
    let career: i32 = career_input.trim().parse().expect("Invalid input");

    // start with the office track, then replace it if another track is chosen
    let mut titles = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    if career == 2 {
        titles = vec!["-", "Research Assistant", "PhD Candidate", "Post-Doc Researcher", "Senior Lecturer", "Dean"];
    } else if career == 3 {
        titles = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", "Senior Associate 3-4", "Partner"];
    } else if career == 4 {
        titles = vec!["Placement", "Classroom Teacher", "Senior Teacher", "Leading Teacher", "Deputy Principal", "Principal"];
    }

    let mut title_input = String::new();
    println!("Enter the job title: ");
    io::stdin().read_line(&mut title_input).expect("Failed to read input");
    let title = title_input.trim().to_lowercase();

    let mut found = false;
    for i in 0..titles.len() {
        if titles[i].to_lowercase() == title {
            println!("{} holds position {}", titles[i], levels[i]);
            found = true;
            break;
        }
    }

    if found == false {
        println!("Not a valid title for that career track.");
    }
}