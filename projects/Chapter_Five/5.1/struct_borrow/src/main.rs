struct Point {
	x: i32,
	y: i32
}

fn main()
{
	let mut p =Point {
		x: 0, 
		y:0
	}; //defien a struct varibale
	//in another word create a struct instance

	let x = &mut p.x; //when borrowed p.x, then p and p.x temporarily lose their permission (but not p.y)

	*x += 1;
	println!("{}, {}", p.x, p.y);
}
