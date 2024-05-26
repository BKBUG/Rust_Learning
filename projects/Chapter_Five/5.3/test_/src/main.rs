fn main()
{
    let text = String::from("hello");
    let text_non_r:&String = &text;
    let text_non_r_two: &String = &text;
    let text_non_r_three:&String = text_non_r;
    println!("the value of text:{}", text);
    println!("the valeu of text_non_r:{text_non_r}");
    println!("the value of text_non_r_new:{}", text_non_r_two);
    println!("the value of text_non_r_three:{text_non_r_three}");
    println!("the value of text:{}", text);
    println!("the valeu of text_non_r:{text_non_r}");
    println!("the value of text_non_r_new:{}", text_non_r_two);
    println!("the value of text_non_r_three:{text_non_r_three}");
}
/* 
fn main()
{
    let mut text = String::from("hello");
    let text_non_r:&mut String = &mut text;
    let text_non_r_two: &String = &text;
    /* 
    let text_non_r_three:&String = text_non_r;
*/
    println!("the value of text:{}", text);
    println!("the value of text_non_r:{text_non_r}")
    /* 
    println!("the value of text_non_r_new:{}", text_non_r_two);
    println!("the value of text_non_r_three:{text_non_r_three}");
*/
}*/