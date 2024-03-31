/*
fn main()
{
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];

    *x += 1;

    println!("{a:?}");
}
*/

/*
fn main()
{
    let mut a = [0, 1, 2, 3];
    let x = &mut a[1];

    let y = &a[2];

    *x += *y;
}
*/

fn main()
{
    let mut a = [0, 1, 2, 3];
    let (a_l, a_r) = a.split_at_mut(2);

    let x = &mut a_l[1];
    let y = &a_r[0];
    *x += *y;

    println!("{a:?}");
}
