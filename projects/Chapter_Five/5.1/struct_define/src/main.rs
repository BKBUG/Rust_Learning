struct User {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}

fn main()
{
	let user1 = User { //immultiple version
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};

	let mut user1 = User { // multiple version
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	
	user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User  //note User is a Struct 
{
	User {
		active: true,
		username: username, //parameter username move to the User.username;
		email: email, //also
		sign_in_count: 1,
	} //note: there no semicon, so this is a expression, not a statement, so it have a value
	// implicitly return User variable instance
}

//use filed init shorthand syntax to rewrite build_user
fn build_user(email: String, username: String) -> User //note the parameter name exactly same the struct User filed name
{
	User {
		active: true,
		username, //also move the parameter username to the User.username, but more convenient
		email,
		sign_in_count: 1,
	}
}
