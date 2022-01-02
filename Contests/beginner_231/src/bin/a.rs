use::std::io;

fn main()
{
    let mut s : String = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");

    let d : i32 = s.trim().parse().expect("Not a number");

    println!("{}", d as f32 / 100.);
}
