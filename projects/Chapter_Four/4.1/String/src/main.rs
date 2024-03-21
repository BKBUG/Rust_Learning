/*
fn main()
{
	let mut s = String::from("hello");
	s.push_str(", world!");
//	println!("s:{s}");
	println!("{}", s);
}
*/


/*
fn main()
{
	let s = String::from("hello"); //s is valid from this point forward

	// do stuff with s
}       // this scope is now over, and s is no longer valid
*/

/*
fn main()
{
	let s1 = String::from("from");
	let s2 = s1;
	println!("{}, world!", s1);
}
*/
// clone of String
fn main()
{
	let x = String::from("hello");
	let y = x.clone();
	println!("x:{}, y:{y}", x);
}
