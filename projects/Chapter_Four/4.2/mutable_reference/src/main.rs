//mutbale reference
/*
fn main()
{
	let mut v: Vec<i32> = vec![1, 2, 3];

	let num: &mut i32 = &mut v[2];// because num is unique reference, so v lose all
	// permission that OWR, 
	//and num gain the O and R(because num not annotated let mut so, it does't have W permission)
	//and *num gain the R and W permission, not have the O permission
	//(that mean cat mutate the data through *num)
	*num += 1; // mutate the data through the *num

	println!("Third element is {}", *num);

	println!("Vector is now {:?}", v);
}
*/
//NOTE: Mutable references allow mutation but prevent aliasing. The borrowed path v becomes tem-
//-porarily unusable, so effectively not a an alias.

//downgrade reference
fn main()
{
	let mut v: Vec<i32> = vec![1, 2, 3];

	let num: &mut i32 = &mut v[2];

	let num2: &i32 = & *num;

	println!("{}, {}", *num, *num2);
}
