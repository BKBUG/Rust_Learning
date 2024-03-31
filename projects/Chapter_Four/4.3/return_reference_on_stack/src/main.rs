fn string_name_with_title(name: &mut Vec<String>) -> String 
{
	name.push(String::from("Esq."));
	let full = name.join(" ");
	full
}

fn main()
{
	let mut name:Vec<String> = vec![String::from("Ferris")];
	let mut first:&mut String = &name[0];

	string_name_with_title(&mut name);
	println!("{}", first);
}
