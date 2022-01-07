use::proconio::input;

fn main()
{
    input!
    {
        n : usize,
        mut w : u64,
        mut c : [[u64; 2]; n],
    }

    c.sort();
    c.reverse();

    let mut ans : u64 = 0;
    let mut i : usize = 0;
    while w > 0 && i < c.len()
    {
        if c[i][1] <= w
        {
            ans += c[i][0] * c[i][1];
            w -= c[i][1];
        }
        else
        {
            ans += c[i][0] * w;
            w = 0;
        }

        i += 1;
    }

    println!("{}", ans);
}

