use proconio::input;
use std::cmp;

fn main()
{
    input!
    {
        n : usize,
        k : i64,
        a : [i64; n]
    }

    let mut under : i64 = 0;
    let mut over : i64 = std::i64::MAX / k;

    while over - under > 1
    {
        let middle : i64 = (under + over) / 2;
        let mut sum : i64 = 0;

        for x in a.iter()
        {
            sum += cmp::min(x, &middle);
        }

        if sum >= k * middle
        {
            under = middle
        }
        else
        {
            over = middle;
        }
    }

    println!("{}", under);
}
