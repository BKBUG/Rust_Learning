/*
fn main()
{
	let mut v: Vec<i32> = vec![1, 2, 3]; //it expression v is mutated

	let num: &i32 = &v[2]; //v is aliased by num

	println!("Third element is {}", *num);

	v.push(4); //it will cause deallocate the origina heap space;
	//then allocate a new large space, and copy the origina data
	//println!(Third element is {}", *num);//it will cause unbehavior
	//because pointer used after its pointee is freed
}
//this programm of the issue is: aliased and mutated in same time
*/
//Pointer Safety Principle: data should never be aliased and mutated at the same time

/*
fn main()
{

	let mut v: Vec<i32> = vec![1, 2, 3]; //because explict annotated "let mut"
	//so v get those permission that OWR

	let num: &i32 = &v[2]; //note:in this line, v lost the W and O permission temporary
	//and num get the R and O permission temporary, *num get the R permission
	println!("Third element is {}", *num); 
	//After this line, then num is no longer in use, so v is longer borrowed,
	//therefore: v regain its WO permission
	//num and *num have lost all of their permission
	println!("Again, the third element is {}", *num);

	v.push(4);

}
*/

fn main()
{
	let mut v:Vec<i32> = vec![1, 2, 3];
	let num : &i32 = &v[2]; //v will be referenced , so v lost the w and o permission
	println!("Third element is {}", *num);
}

//version: complier error
fn main() 
{
	let mut v: Vec<i32> = vec![1, 2, 3];
	let num: &i32 = &v[2];

	v.push(4); //v.push expect the W permission, but v borrowed, so it lost the W permission, so it error
	//NOTE: Rust disallow the both alias and mutated exist in same time
	println!("Third element is {}", *num);
}