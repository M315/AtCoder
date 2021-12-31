use std::io;

fn main()
{
    // Read the strings
    let mut s : String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    
    let mut t : String = String::new();
    io::stdin()
        .read_line(&mut t)
        .expect("Failed to read line");


    // Convert the string to bytes
    let mut s_bytes = s.bytes();
    let mut t_bytes = t.bytes();

    // Compute the difference of the fisrt to characters
    let k : u8 = (26 + t_bytes.next().unwrap() - s_bytes.next().unwrap()) % 26;

    // For each character compare if the difference is the same
    for _ in 1..s.len() - 1
    {
        if k != (26 + t_bytes.next().unwrap() - s_bytes.next().unwrap()) % 26
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
