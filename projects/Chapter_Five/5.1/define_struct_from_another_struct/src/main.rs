struct User {
	active: bool,
	username:String,
	email: String,
	sign_in_count: u64,
}

fn main()
{
	let user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,
	};
	//snip

	let user2 = User { 
		active: user1.active,
		username: user1.username,//note:there result user1.usename is invalid, because user1 move to user2
		email: String::from("another@exanple.com"),
		sign_in_count: user1.sign_in_count,
	};
	/*
//	println!("value of user1.username{}", user1.username);
	println!("value of user1.active {}", user1.active);
	println!("value of user1.email {}", user1.email);
	println!("value of user1.sign_in_count {}", user1.sign_in_count);
*/

	println!("value of user2.username {}", user2.username);
	println!("value of user2.active {}", user2.active);
	println!("value of user2.email {}", user2.email);
	println!("value of user2.sign_in_count {}", user2.sign_in_count);

	println!("next define user3");
	//version use struct update syntax
	let user3 = User {
		email: String::from("another@example.com"),
		..user2 //express the remaining filed have same value of the given struct instance user1
			//in another word is use3.active, user3.username, user3.sign_in_count have same value with
			//user2;

			//so that cause the user2.username is invalid, syntax .. 
			//move the value user2.username to the user3.username
	};
//	println!("value of user2.username {}", user2.username);
	println!("value of user2.active {}", user2.active);
	println!("value of user2.email {}", user2.email);
	println!("value of user2.sign_in_count {}", user2.sign_in_count);
	println!("");
	println!("value of user3.username {}", user3.username);
	println!("value of user3.active {}", user3.active);
	println!("value of user3.email {}", user3.email);
	println!("value of user3.sign_in_count {}", user3.sign_in_count);
}


