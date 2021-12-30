use proconio::input;
use std::collections::HashMap;

fn main()
{
    input!
    {
        n : usize,
        k : i64,
        a : [i64; n]
    }

    let mut s : Vec<i64> = Vec::new();
    
    // Compute comulative sums
    s.push(0);
    for (i, x) in a.iter().enumerate()
    {
        s.push(s[i] + x);
    }

    let mut map : HashMap<i64, i64> = HashMap::new();
    let mut ans : i64 = 0;

    for i in 1..=n
    {
        *map.entry(s[i - 1]).or_insert(0) += 1;
        ans += *map.get(&(s[i] - k)).unwrap_or(&0);
    }

    println!("{}", ans);
}
