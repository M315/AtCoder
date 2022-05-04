use proconio::input;

fn main() {
    input!{
        n : usize,
        t : [i64; n]
    }

    let mut ans : i64 = 0;

    for i in 0..n {
        let pt : i64 = 1 << t[i];
        ans = (ans / pt + 1) * pt;
        if ans % (pt * 2) == 0 {
            ans += pt;
        }
    }

    println!("{}", ans);
}
