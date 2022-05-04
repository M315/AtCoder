use proconio::input;
use std::cmp::min;

fn main() {
    input!{
        n : usize,
        k : usize,
        a : [i32; n]
    }

    let mut b : Vec<(i32, usize)> = Vec::new();
    for i in 0..n {
        b.push((a[i], i));
    }

    b.sort_by_key(|&x| (-x.0, x.1));

    let mut ans : i32 = std::i32::MAX;
    let mut m = std::i32::MAX;

    for p in b {
        if p.1 < k {
            ans = min(ans, m - p.1 as i32);
        } else {
            m = min(m, p.1 as i32);
        }
    }

    if ans > n as i32 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
