/*
fn five() -> i32 {
	//explicit return type
	5//note: the value five not have semicolon, 
	//if have semicolon express it is a statement not a expression;
	//and a statement do not return a value 
}

fn main()
{
	let x = five();

	println!("The value of x: {x}");
}
*/

fn main()
{
	let x = plus_one(5);
	println!("The value of x is:{}", x);
}

fn plus_one(x:i32) -> i32 { //parameter type i32, return type i32
	x + 1
}
