/*
#![allow(unused)]
fn main()
{
    fn add_big_string(dst: &mut Vec<String>, src: &[String]) {
        let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
        for s in src {
            if s.len() > largest.len() {
                dst.push(s.clone());
            }
        }
    }
}
*/

/*
#![allow(unused)]
fn main()
{
    fn add_big_string(dst: &mut Vec<String>, src: &[String]) {
        let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
        let to_add: Vec<String> = 
            src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
        dst.extend(to_add);
    }
}
*/


fn main()
{
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    println!("{s_ref}!");


    let v: Vec<String> = vec![String::from("Hello world")];
    let mut s:String = v[0].clone();
    s.push('!');
    println!("{s}");


    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(0 == v.len());
}
