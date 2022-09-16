#[derive(PartialEq, PartialOrd)]
struct Fraction {
	top: u128,
	bottom: u128,
}

impl Fraction {
	fn powi(self, power: u32) -> Self {
		let new_top = self.top.pow(power);
		let new_bottom = self.bottom.pow(power);
		Fraction { top: new_top, bottom: new_bottom }
	}

	fn divide(&mut self, number: u128) {
		let top_div = self.top / number;
		if self.top == (top_div * number) {
			self.top = top_div
		} else {
			self.bottom *= number
		}
	}

	fn minus(&mut self, number: u128) {
		self.top -= self.bottom * number;
	}
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let top = self.top;
		let bottom = self.bottom;
        write!(f, "{top}/{bottom}")
    }
}

fn main() {
	const BASE: Fraction = Fraction{top: 3, bottom: 2};
	let mut power = 1;
	let mut numbers = vec![BASE];
	loop {
		power += 1;
		let mut new = BASE.powi(power);

		let two = Fraction{top: 2, bottom: 1};
		while new >= two {
			new.divide(2);
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
