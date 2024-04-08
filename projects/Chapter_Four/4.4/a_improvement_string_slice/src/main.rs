fn main()
{
	let my_string = String::from("hello world");


	//'first_word' works on slices of 'String' s, whether partial or whole
	let word = first_word(&my_string[0..6]); // pass my_string of slice begging 0 to 6
	let word = first_word(&my_string[..]); //pass the entire my_string
	// 'first_word' also works on reference to 'String's, which are equivalent
	// to whole slices of 'String's
	let word = first_word(&my_string);

	let my_string_literal = "hello world";

	//'first_word' works on slices of string literals, whether partial or whole
	let word = first_word(&my_string[0..6]);
	let word = first_word(&my_string[..]);


	//Because string literals *are* string slices already 
	//this works too, without the slice syntax!
	let word = first_word(my_string_literal);
}

fn first_word(s:&str)-> &str
{
	let bytes = s.as_bytes();

	for (i, &iter) in bytes.iter().enumerate() {
		if iter == b' ' {
			return &s[0..i];
		}
	}
	s[..]
}
