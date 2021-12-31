use std::io;

fn main()
{
    let mut s : String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    let mut chars = s.chars();

    let a : u32 = chars.next().expect("Missing first digit!").to_digit(10).expect("Not a number");

    assert!(!chars.next().is_none());
 
    let b : u32 = chars.next().expect("Missing second digit!").to_digit(10).expect("Not a number");

    println!("{}", a * b);
}
