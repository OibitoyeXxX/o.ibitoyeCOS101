struct Laptop {
    brand: String,
    price: u32,
}

impl Laptop {

    fn cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    let hp = Laptop { brand: String::from("HP"), price: 650_000 };
    let ibm = Laptop { brand: String::from("IBM"), price: 755_000 };
    let toshiba = Laptop { brand: String::from("Toshiba"), price: 550_000 };
    let dell = Laptop { brand: String::from("Dell"), price: 850_000 };


    let quantity = 3;

    println!("=== ALABA ELECTRONICS ===");
    println!("{} x {} = N{}", quantity, hp.brand, hp.cost(quantity));
    println!("{} x {} = N{}", quantity, ibm.brand, ibm.cost(quantity));
    println!("{} x {} = N{}", quantity, toshiba.brand, toshiba.cost(quantity));
    println!("{} x {} = N{}", quantity, dell.brand, dell.cost(quantity));

    let total = hp.cost(quantity) + ibm.cost(quantity) + toshiba.cost(quantity) + dell.cost(quantity);
    println!("-------------------------");
    println!("TOTAL COST = N{}", total);
}