/*
fn main()
{
	let s = String::from("hello world");
	
	let hello: &str = &s[0..5];
	let world: &str = &s[6..11];
	let s2: &String = &s;
}
*/

/*
fn main()
{
	let mut s = String::from("hello");
	
	let hello: &str = &s[0..5];

	println!("{hello}");
	s.push_str(" world");
}
*/

fn first_word(s: &String) -> &str //The type taht signifies "string slice" is written as &str
{
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i]; //slice the string s;
		}
	}
	
	&s[..] //return entire string // equal the s[0..s.len]

}


fn main()
{
	let mut s = String::from("hello world");
	let word = first_word(&s);
	s.clear();
	println!("the first word is: {}", word);
}
