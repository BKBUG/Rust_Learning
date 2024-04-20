/*
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle { //define method of struct Rectangle
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

fn main()
{
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("The area of the rectangle is {} square pixels.", rect1.area());// call the metho of struct Rectangle
}
*/

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn width(&self) -> bool {
		self.width > 0
	}
}

fn main()
{
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	if rect1.width()  {//call the method of width
		println!("The rectangle has a nonzero width; it is {}", rect1.width); //access the data member width, not method
	}
}
