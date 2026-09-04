fn main () {
	let toshiba:f64 = 2.0 * 450_000.00; 
	let mac: f64 = 1.0 * 1_500_000.00;
	let hp :f64 = 3.0 * 750_000.00;
	let dell : f64 = 3.0 * 2_850_000.00;
	let acer : f64 = 1.0 * 250_000.00;

	// sum and average 
	let sum = toshiba + mac + hp + dell + acer ;
	println! ("The sum is {}", sum);
	let average = sum/ 5.0;
	println! ("The average is {}", average);
}