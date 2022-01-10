use proconio::input;

fn main()
{
    input!
    {
        n : u64,
    }

    let mut count : u64 = 0;
    let mut a : u64 = 1;
    
    while a * a * a <= n
    {
        let mut b : u64 = a;

        while a * b * b <= n
        {
            count += (n / (a * b)) - b + 1;
            b += 1;
        }

        a += 1;
    }

    println!("{}", count);
}
