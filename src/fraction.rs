#[derive(PartialEq, Clone, Debug)]
pub struct Fraction {
	top: u128,
	bottom: u128,
}

impl Fraction {
	pub const fn new(top: u128, bottom: u128) -> Self {
		let gcd = gcd(top, bottom);
		Self {
			top: top / gcd,
			bottom: bottom / gcd,
		}
	}

	const fn simplify(self) -> Self {
		let gcd = gcd(self.top, self.bottom);
		Self {
			top: self.top / gcd,
			bottom: self.bottom / gcd,
		}
	}

	fn simplify_mut(&mut self) {
		let gcd = gcd(self.top, self.bottom);
		self.top = self.top / gcd;
		self.bottom = self.bottom / gcd;
	}

	pub fn powi(self, power: u32) -> Self {
		let new_top = self.top.pow(power);
		let new_bottom = self.bottom.pow(power);
		Self::new(new_top, new_bottom)
	}
}

impl std::fmt::Display for Fraction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let top = self.top;
		let bottom = self.bottom;
		write!(f, "{top}/{bottom}")
	}
}

impl std::ops::Add for Fraction {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		let lcm = lcm(self.bottom, rhs.bottom);
		let first_top = self.top * (lcm / self.bottom);
		let second_top = rhs.top * (lcm / rhs.bottom);
		let top = first_top + second_top;
		Self::new(top, lcm)
	}
}

impl std::ops::AddAssign for Fraction {
	fn add_assign(&mut self, rhs: Self) {
		let lcm = lcm(self.bottom, rhs.bottom);
		self.top *= lcm / self.bottom;
		self.top += rhs.top * (lcm / rhs.bottom);
		self.bottom = lcm;
		self.simplify_mut();
	}
}

impl std::ops::Sub for Fraction {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		let lcm = lcm(self.bottom, rhs.bottom);
		let first_top = self.top * (lcm / self.bottom);
		let second_top = rhs.top * (lcm / rhs.bottom);
		let top = first_top - second_top;
		Self::new(top, lcm)
	}
}

impl std::ops::Mul for Fraction {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		let top = self.top * rhs.top;
		let bottom = self.bottom * rhs.bottom;
		Self::new(top, bottom)
	}
}

impl std::ops::Div for Fraction {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		let top = self.top * rhs.bottom;
		let bottom = self.bottom * rhs.top;
		Self::new(top, bottom)
	}
}

impl std::ops::DivAssign for Fraction {
	fn div_assign(&mut self, rhs: Self) {
		self.top = self.top * rhs.bottom;
		self.bottom = self.bottom * rhs.top;
		self.simplify_mut();
	}
}

impl<T> std::ops::DivAssign<T> for Fraction
where
	u128: std::ops::MulAssign<T>,
{
	fn div_assign(&mut self, rhs: T) {
		self.bottom *= rhs;
		self.simplify_mut();
	}
}

impl std::ops::Mul<u128> for Fraction {
	type Output = Self;

	fn mul(self, rhs: u128) -> Self::Output {
		let gcd = gcd(self.top, rhs);
		let leftover = rhs / gcd;

		let top = self.top * leftover;
		let bottom = self.bottom / gcd;
		Self { top, bottom }
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
const fn gcd(n1: u128, n2: u128) -> u128 {
	let (mut x, mut y) = if n1 > n2 { (n1, n2) } else { (n2, n1) };

	let mut remainder = x % y;

	while remainder != 0 {
		x = y;
		y = remainder;
		remainder = x % y;
	}
	y
}

/// Lowest Common Multiple
const fn lcm(n1: u128, n2: u128) -> u128 {
	(n1 * n2) / gcd(n1, n2)
}

#[test]
fn add_one_denominator_change() {
	let frac1 = Fraction::new(3, 4);
	let frac2 = Fraction::new(5, 8);
	assert_eq!(frac1 + frac2, Fraction::new(11, 8))
}

#[test]
fn add_denominators_multiply() {
	let frac1 = Fraction::new(3, 4);
	let frac2 = Fraction::new(4, 5);
	assert_eq!(frac1 + frac2, Fraction::new(31, 20))
}

#[test]
fn add_lcm() {
	let frac1 = Fraction::new(5, 6);
	let frac2 = Fraction::new(3, 4);
	assert_eq!(frac1 + frac2, Fraction::new(19, 12))
}

#[test]
fn add_simplify() {
	let frac1 = Fraction::new(4, 3);
	let frac2 = Fraction::new(2, 3);
	assert_eq!(frac1 + frac2, Fraction::new(2, 1))
}

#[test]
fn add_assign_one_denominator_change() {
	let mut fraction = Fraction::new(3, 4);
	fraction += Fraction::new(5, 8);
	assert_eq!(fraction, Fraction::new(11, 8))
}

#[test]
fn add_assign_denominators_multiply() {
	let mut fraction = Fraction::new(3, 4);
	fraction += Fraction::new(4, 5);
	assert_eq!(fraction, Fraction::new(31, 20))
}

#[test]
fn add_assign_lcm() {
	let mut fraction = Fraction::new(5, 6);
	fraction += Fraction::new(3, 4);
	assert_eq!(fraction, Fraction::new(19, 12))
}

#[test]
fn add_assign_simplify() {
	let mut fraction = Fraction::new(4, 3);
	fraction += Fraction::new(2, 3);
	assert_eq!(fraction, Fraction::new(2, 1))
}

#[test]
fn sub_one_denominator_change() {
	let frac1 = Fraction::new(3, 4);
	let frac2 = Fraction::new(5, 8);
	assert_eq!(frac1 - frac2, Fraction::new(1, 8))
}

#[test]
fn sub_denominators_multiply() {
	let frac1 = Fraction::new(4, 5);
	let frac2 = Fraction::new(3, 4);
	assert_eq!(frac1 - frac2, Fraction::new(1, 20))
}

#[test]
fn sub_lcm() {
	let frac1 = Fraction::new(5, 6);
	let frac2 = Fraction::new(3, 4);
	assert_eq!(frac1 - frac2, Fraction::new(1, 12))
}

#[test]
fn sub_simplify() {
	let frac1 = Fraction::new(4, 3);
	let frac2 = Fraction::new(1, 3);
	assert_eq!(frac1 - frac2, Fraction::new(1, 1))
}
