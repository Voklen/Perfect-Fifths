#[derive(PartialEq, Clone)]
struct Fraction {
	top: u128,
	bottom: u128,
}

impl Fraction {
	fn powi(self, power: u32) -> Self {
		let new_top = self.top.pow(power);
		let new_bottom = self.bottom.pow(power);
		let gcd = gcd(new_top, new_bottom);
		Fraction {
			top: new_top / gcd,
			bottom: new_bottom / gcd,
		}
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

impl PartialOrd for Fraction {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if self.top == other.top && self.bottom == other.bottom {
			return Some(std::cmp::Ordering::Equal);
		}
		let first = (self.top as f64) / (self.bottom as f64);
		let second = (other.top as f64) / (other.bottom as f64);

		if second > first {
			return Some(std::cmp::Ordering::Greater);
		} else {
			return Some(std::cmp::Ordering::Less);
		}
	}
}

/// Greatest Common Denominator
fn gcd(n1: u128, n2: u128) -> u128 {
	let (mut x, mut y) = if n1 > n2 { (n1, n2) } else { (n2, n1) };

	let mut remainder = x % y;

	while remainder != 0 {
		x = y;
		y = remainder;
		remainder = x % y;
	}
	y
}

fn contains(nums: &Vec<Fraction>, num: Fraction) -> Option<usize> {
	let mut highest = nums.len();
	let mut lowest = 0;
	while highest > lowest {
		let pos = (highest + lowest) / 2;
		if num < nums[pos] {
			lowest = pos + 1
		} else if num > nums[pos] {
			highest = pos - 1
		} else {
			return Some(pos);
		}
	}
	None
}

fn main() {
	const BASE: Fraction = Fraction { top: 3, bottom: 2 };
	let mut power = 1;
	let mut numbers = vec![BASE];
	loop {
		power += 1;
		let mut new = BASE.powi(power);

		let two = Fraction { top: 2, bottom: 1 };
		while new >= two {
			new.divide(2);
		}
		println!("Power {power}: {new}");
		match contains(&numbers, new.clone()) {
			Some(i) => {
				println!("FOUND IT! It's {i} and {power}");
				break;
			}
			None => {}
		}
		numbers.push(new);
	}
}
