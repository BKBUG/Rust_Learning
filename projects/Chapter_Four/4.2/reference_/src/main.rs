//error version
/*
fn main()
{
	let m1 = String::from("Hello");
	let m2 = String::from("world");
	greet(m1, m2);
	let s = format!("{} {}", m1, m2);
}
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
fn main()
{
	let m1 = String::from("Hello");
	let m2 = String::from("world");
	greet(&m1, &m2); // note the ampersands
	let s = format!("{} {}", m1, m2);
}

fn greet(g1:&String, g2:&String)
{
	println!("{} {}!", g1, g2);
}
*/

//dereference example 
/*
{
let mut x:Box<i32> = Box::new(1);
let a: i32 = *x; //*x reads the heap value, so a = 1
*x += 1; //*x on the left-side modifies the heap value, so x points to the value 2

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

