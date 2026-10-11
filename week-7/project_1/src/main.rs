use std::io;

fn main() {
    println!("========================================");
    println!("        OTITO'S SHAPE CALCULATOR");
    println!("========================================");
    println!("1 - Trapezium (area)");
    println!("2 - Rhombus (area)");
    println!("3 - Parallelogram (area)");
    println!("4 - Cube (surface area)");
    println!("5 - Cylinder (volume)");
    println!("========================================");

    let mut choice_input = String::new();
    println!("Enter your choice (1-5):");
    io::stdin().read_line(&mut choice_input).expect("Failed to read input");
    let choice: i32 = choice_input.trim().parse().expect("Invalid input");

    if choice == 1 {
        let mut input1 = String::new();
        println!("Enter height:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let height: f64 = input1.trim().parse().expect("Invalid input");

        let mut input2 = String::new();
        println!("Enter base1:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let base1: f64 = input2.trim().parse().expect("Invalid input");

        let mut input3 = String::new();
        println!("Enter base2:");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let base2: f64 = input3.trim().parse().expect("Invalid input");

        let area = height / 2.0 * (base1 + base2);
        println!("Area of the trapezium is {}", area);
    } else if choice == 2 {
        let mut input1 = String::new();
        println!("Enter diagonal1:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let diagonal1: f64 = input1.trim().parse().expect("Invalid input");

        let mut input2 = String::new();
        println!("Enter diagonal2:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let diagonal2: f64 = input2.trim().parse().expect("Invalid input");

        let area = 0.5 * diagonal1 * diagonal2;
        println!("Area of the rhombus is {}", area);
    } else if choice == 3 {
        let mut input1 = String::new();
        println!("Enter base:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let base: f64 = input1.trim().parse().expect("Invalid input");

        let mut input2 = String::new();
        println!("Enter altitude:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let altitude: f64 = input2.trim().parse().expect("Invalid input");

        let area = base * altitude;
        println!("Area of the parallelogram is {}", area);
    } else if choice == 4 {
        let mut input1 = String::new();
        println!("Enter side:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let side: f64 = input1.trim().parse().expect("Invalid input");

        let surface_area = 6.0 * side * side;
        println!("Surface area of the cube is {}", surface_area);
    } else if choice == 5 {
        let pi: f64 = 3.14159;

        let mut input1 = String::new();
        println!("Enter radius:");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let radius: f64 = input1.trim().parse().expect("Invalid input");

        let mut input2 = String::new();
        println!("Enter height:");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let height: f64 = input2.trim().parse().expect("Invalid input");

        let volume = pi * radius * radius * height;
        println!("Volume of the cylinder is {}", volume);
    } else {
        println!("Invalid choice. Pick a number from 1 to 5.");
    }
}