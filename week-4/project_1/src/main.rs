use std::io;

fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    println!("Enter a->");
    io::stdin().read_line(&mut input_a).expect("Not a valid string");
    let a: f64 = input_a.trim().parse().expect("Not a valid number");

    println!("ENTER b->");
        io::stdin().read_line(&mut input_b).expect("Not a valid string");
        let b: f64 = input_b.trim().parse().expect("Not a valid number");

    println!("Enter c->");
    io::stdin().read_line(&mut input_c).expect("Not a value string");
    let c: f64 =input_c.trim().parse().expect("Not a valid number");

    let d: f64 = b * b - 4.0 * a * c ;

    if d > 0.0 {
        let root1 =(-b + d.sqrt()) / (2.0 * a);
        let root2 =(-b - d.sqrt()) / (2.0 * a);
        println!("Two distinct roots ->{} and {}", root1, root2);
    } else if d == 0.0 {
        let root = -b / (2.0 * a);
        println!("Only one real root -> {}", root );
    }else {
        println!("No real roots");
    }

}
