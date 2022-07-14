use num::Num;

/// Defines a rectangle.
#[derive(Clone)]
pub struct Rect<T> {
	pub x: T,
	pub y: T,
	pub width: T,
	pub height: T,
}

impl<T> Rect<T> {
	pub fn new(x: T, y: T, width: T, height: T) -> Rect<T> {
		Rect {x, y, width, height}
	}
}

impl<T: Ord + Num + Copy> Rect<T> {
	pub fn clamp(&self, other: Self) -> Self {
		Rect {
			x: self.x.clamp(other.x, other.width),
			y: self.y.clamp(other.y, other.height),
			width: (other.width + other.x - self.x).min(self.width - self.x).clamp(other.x, self.width),
			height: (other.height + other.y - self.y).min(self.height - self.y).clamp(other.y, self.height),
		}
	}
	
	pub fn distance_to_border(&self, other: Self) -> (T, T) {
		let zero = T::zero();
		
		(
			(self.x - other.x).min(zero) + (self.x - other.x - other.width).max(zero),
			(self.y - other.y).min(zero) + (self.y - other.y - other.height).max(zero),
		)
	}
	
	pub fn x2(&self) -> T {
		self.x + self.width
	}
	
	pub fn y2(&self) -> T {
		self.y + self.height
	}
}