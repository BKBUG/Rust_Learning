/*
fn main()
{
//	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let test_tup: (i32, f64, u8, i32) = (500, 6.4, 1, 10);
}
*/

fn main()
{
	let tup = (500, 6.4, 1);
	let (x, y, z) = tup;
	println!("The value of y is:{y}");
	println!("The value of x is:{}", x);
}
