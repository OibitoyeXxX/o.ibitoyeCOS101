use std::io;
fn main() {
    println!("----------------------------");
    println!("WELCOME TO OTITO'S W EATS ");
    println!("----------------------------");
    println!("            MENU            ");
    println!("P - Poundo Yam / Edinkaiko Soup    N3,200");
    println!("F - Fried Rice & Chicken           N3,000");
    println!("A - Amala & Ewedu Soup             N2,500");
    println!("E - Eba & Egusi Soup               N2,000");
    println!("W - White Rice & Stew              N2,500");
    println!("============================================");

    let mut food = String::new();
    println!("Enter food choice (|P|F|A|E|W|)");
    io::stdin().read_line(&mut food).expect("Not a valid input");
    let food = food.trim().to_uppercase();

    let mut quantity = String::new();
    println!("How many portions?");
    io::stdin().read_line(&mut quantity).expect("Not a valid input");
    let quantity: f64 = quantity.trim().parse().expect("Not a valid number");

    let price: f64;

    if food == "P" {
        price = 3200.0;
    } else if food == "F" {
        price = 3000.0;
    } else if food == "A" {
        price = 2500.0;
    } else if food == "E" {
        price = 2000.0;
    } else if food == "W" {
        price = 2500.0;
    } else {
        println!("Invalid food code.");
        price = 0.0;
    }

    let mut total = price * quantity;

    if total > 10000.0 {
        total = total * 0.95;
    }
    println!("Total is N{}", total);
}