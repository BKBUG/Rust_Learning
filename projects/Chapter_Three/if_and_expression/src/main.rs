/*
fn main()
{
	let condition = true;
	let number = if condition {5} else {6}; //note if is a expression, so it can return a value

	println!("The value of number is: {number}");
}
*/


fn main()
{
	let condition = true;

	let number = fi condition {5} else {"six"};
	
	println!("The value of number is:{number}");//values that have the potential to be results from each arm of the if 
						// must be the same type, so it will error
}
