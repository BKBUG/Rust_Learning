/*
fn main() 
{
	let s = String::from("hello");
	let s2;
	let b = false;

	if b {
		s2 = s;	
		println!{"here expression in if "};
	}
	else {
		println!{"Here expression in else"};
	}

//	println!("{}", s);
}
*/


//version directly print the value of Box::new(number) test

fn main()
{
	let x = Box::new(1);
	let y = Box::new(&x);
	println!("{}", x);
	println!("{}", y);
	println!("{}", **y);
	println!("{}", ***y);
}
