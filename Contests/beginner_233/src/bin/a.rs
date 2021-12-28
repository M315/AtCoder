use proconio::input;

fn ceil(a : i32, b : i32) -> i32
{
    (a + b - 1) / b
}

fn main()
{
    input!
    {
        x : i32,
        y : i32,
    }

    if x >= y
    {
        println!("0");
    }
    else
    {
        println!("{}", ceil(y - x, 10));
    }
}
