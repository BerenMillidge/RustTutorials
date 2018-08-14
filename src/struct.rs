#[derive(Debug)]

struct Rectangle {
	width: u32, 
	height: u32
}
// to define a method on a struct  - i.e. more like a standard objec, you use the impl keyword block
impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		return self.width > other.width && self.height > other.height;
	}
}


fn main() {
	let width1 = 30;
	let height1 = 50;
	let rect = (30,50);

	println!("Area: {}", area(width1, height1));
	println!("Area: {}", area_tuple(rect));
	let rectangle = Rectangle { width: 30, height: 50};
	let rect2 = Rectangle { width: 10, height: 10};
	println!("rectangle is {:?}", rectangle);
	println!("{}",rectangle.area());
	println!("{}",rectangle.can_hold(&rect2));
}	



fn area(width: u32, height:u32) -> u32 {
	width * height
}
fn area_tuple(dimensions: (u32, u32)) -> u32 {
	dimensions.0 * dimensions.1
}