struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main()
{
	let black = Color(0, 0, 0);
	let origin = Point(0,0,0);
	//note:the black and origin valuse are different types because they are instance of different tuple structs
}
