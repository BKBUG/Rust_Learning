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

/* 
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
*/

/*
#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main()
{
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};

	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

impl Rectangle {
	fn area(&self) ->u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

impl Rectangle {
	fn square(size: u32) -> Self {
		Self {
			width: size,
			height: size,
		}
	}
}

//equal previous code that impl Rectangle  {fn area{...}, can_hold{}}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
}

impl Rectangle {
	fn can_hold (&self, other:&Rectangle) -> bool {
		self.width > other.width && self.height > other.height
	}
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}
	fn set_width(&mut self, width: u32) {
		self.width = width;
	} 
}
*/


#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn set_width(&mut self, width: u32) {
		self.width = width;
	}
}

/*
fn main()
{
	let mut r = Rectangle {
		height: 1,
		width: 2,
	};

	let area1 = r.area();
	let area2 = Rectangle::area(&r);
	assert_eq!(area1, area2);

	r.set_width(2);
	Rectangle::set_width(&mut r, 2);
}
*/

fn main() 
{
	let r = &mut Box::new(Rectangle {
		width: 1,
		height: 2
	});
	let area1 = r.area();//when call the area() method,
	//the mut& downgrade into shared reference
	//because area except a immutable reference
	//note: here dereference twice
	//first dereference from the heap
	//second dereference from the mutable & operator
	let area2 = Rectangle::area(&**r);
	assert_eq!(area1, area2);

}

































