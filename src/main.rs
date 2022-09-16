fn approx_equal(a: &f64, b: f64, decimal_places: u8) -> bool {
	let factor = 10.0f64.powi(decimal_places as i32);
	let a = (a * factor).trunc();
	let b = (b * factor).trunc();
	a == b
}

fn main() {
	const BASE: f64 = 1.5;
	let mut power = 1;
	let mut numbers = vec![BASE];
	loop {
		power += 1;
		let mut new = BASE.powi(power);

		while new >= 2.0 {
			new /= 2.0;
		}
		println!("Power {power}: {new}");
		let result = numbers
			.clone()
			.into_iter()
			.find(|number| approx_equal(number, new, 4));
		match result {
			Some(i) => {
				println!("FOUND IT! It's {i} and {power}");
				break;
			}
			None => {}
		}
		numbers.push(new);
	}
}
