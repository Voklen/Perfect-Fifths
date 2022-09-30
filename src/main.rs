mod fraction;
use fraction::Fraction;

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
	const BASE: Fraction = Fraction::new(3, 2);
	let mut power = 1;
	let mut numbers = vec![BASE];
	loop {
		power += 1;
		let mut new = BASE.powi(power);

		let two = Fraction::new(2, 1);
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
