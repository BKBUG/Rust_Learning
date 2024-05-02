//error version
/*
fn main()
{
	let m1 = String::from("Hello");
	let m2 = String::from("world");
	//because the Type String is not implement copy trait
	//so call when the greet(g1:String, g2:String)
	//m1 and m2 moved into to the parameter g1 and g2, so m1 and m2 become invalid
	greet(m1, m2);
	//g1 is a reference that points to m1 on the stack;
	//and m1 is a String containing a box that points to "Hello" on the heap
	//note!: While m1 owns the  heap data "Hello", g1 does not own either m1 or "Hello"
	//so, we can see that a reference don't have the Owner permission
	let s = format!("{} {}", m1, m2);
}
 //Remember:"Reference are non-owning pointers, because they do not own the data they point to."
 fn greet(g1:String, g2:String)  
 {
	println!("{} {}!", g1, g2) ;
 }
 */

 //return a tuple version
 /*
fn main()
{
	let m1 = String::from("hello");
	let m2 = String::from("world");
	let (m1_again, m2_again) = greet(m1, m2);
	let s= format!("{} {}", m1_again, m2_again);
}

fn greet(m1:String, m2:String) -> (String, String)
{
	println!("{} {}!", m1, m2);
	(m1, m2)
}
*/

//reference version
/*
//note: A reference is a kind of pointer. not like c plus plus that refernece is a label
fn main()
{
	let m1 = String::from("Hello");
	let m2 = String::from("world");
	greet(&m1, &m2); // note the ampersands
	let s = format!("{} {}", m1, m2);
}

fn greet(g1:&String, g2:&String) //the greet parameter g1 is changed to &String
{									//meaning "a reference to a String"
	println!("{} {}!", g1, g2);
}
//Again: Reference are non-owning pointers, because they do not own the data they point to.
*/

//dereference example 
/*
{
let mut x:Box<i32> = Box::new(1);
let a: i32 = *x; //*x reads the heap value, so a = 1 
*x += 1; //*x on the left-side modifies the heap value, so x points to the value 2

//because refernece is kind of a pointer, so need use & that meaning get the address of x
let r1: &Box<i32> = &x; // r1 points to x on the stack, //so r1 is a pointer of pointer
let b:i32 = **r1; //two dereference get us to the heap value

let r2: &i32 = &*x; //r2 points to the heap valur directly // so r2 is pointer to point a value that type is i32?
			// note r2 directly point to the value that on the heap, not stack
let c: i32 = *r2; //so only one dereference is needed to read it


let x: Box<i32> = Box::new(-1);
let x_abs1 = i32::abs(*x); //explicit dereference
let x_abs2 = x.abs(); //implicit dereference

assert_eq!(x_abs1, x_abs2);

let r:&Box<i32> = &x; //r is pointer to point a value that type is Box<i32>?
let r_abs1 = i32::abs(**x) //explicit dereference (twice)
let r_abs2 = r.abs(); //implicit dereference (twice)
assert_eq!(r_abs1, r_abs2);

let s = String::from("Hello");
let s_len1 = str::len(&s); //explicit reference
let s_len2 = s.len(); //implicit reference
assert_eq!(s_len1, s_len2);
}
*/

