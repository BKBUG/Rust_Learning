fn first_word(s:&String) -> usize 
{
    let bytes = s.as_bytes(); // convert String to an array of bytes;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { //if find a space in the s, than return the index
            return i;
        }
    }

    s.len() //if s don't have a space, than return the size of s;
}

fn main()
{}
