/*
fn main()
{
	let number = 3;

	if number < 5 {
		println!{"condition was true"};
	}
	else {
		println!{"condition was false"}	;
	}
}
*/
/*
fn main ()
{
	let number = 4;
	if number { //condition must be a booleaning value
		println!{"condition was true"};
	}
	else {
		println!{"condition was false"};
	}
}
*/

fn main()
{
	let number = 6;

	if number % 4 == 0 {
		println!{"number is divisible by 4"};
	}
	else if number % 3 == 0 {
		println!("number is divisible by 3");
	}
	else if number % 2 == 0 {
		println!("number is divisible bby 2");
	}
	else {
		println!("number is not divisible by 4, 3, or 2");
	}
}
