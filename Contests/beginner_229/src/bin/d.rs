use::proconio::input;
use::proconio::marker::Chars;
use::std::cmp;

fn main()
{
    input!
    {
        s : Chars,
        k : u64,
    }

    let n : usize = s.len();
    let mut count : Vec<u64> = vec![0; n + 1];

    for i in 0..n
    {
        if s[i] == '.'
        {
            count[i + 1] = count[i] + 1;
        }
        else
        {
            count[i + 1] = count[i];
        }
    }

    let mut ans : usize = 0;
    let mut r : usize = 0;

    for l in 0..n
    {
        while r <= n - 1 && count[r + 1] - count[l] <= k
        {
            r += 1;
        }
        ans = cmp::max(ans, r - l);
    }

    println!("{}", ans);
}
