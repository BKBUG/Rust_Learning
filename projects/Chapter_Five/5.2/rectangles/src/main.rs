/*
fn main ()
{
	let width1 = 30;
	let height1 = 50;

	println!("The area of the rectangle is {} square pixels.",
		area(width1, height1));
}

fn area(width1: u32, height1: u32) -> u32
{
	width1 * height1
}
*/


/*
//tuple version
fn main()
{
	let rect1 = (30, 50); //a tuple

	println!("use tuple version");
	println!("The area of the rectangle is {} square pixels.", area(rect1));
}
 fn area(dimensions:(u32, u32)) -> u32
 {
	dimensions.0 * dimensions.1
 }
 */

/*
//struct version
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

	println!("use struct version");
       	println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 
{
	rectangle.height * rectangle.width	
}
*/

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

fn main()
{
	let scale = 2;
	let rect1 = Rectangle {
		width : dbg!(30 * scale),
		height: 50,
	};

	dbg!(&rect1);
}
