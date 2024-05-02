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

	let mut v: Vec<i32> = vec![1, 2, 3];

	let num: &i32 = &v[2];

	println!("Third element is {}", *num);
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
