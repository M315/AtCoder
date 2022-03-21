use proconio::input;

fn main() {
    input! {
        n : usize,
    }

    let m : usize = 998_244_353;
    let mut d : [usize; 9] = [1; 9];
    let mut p : [usize; 9] = [1; 9];

    for _ in 2..=n {
        d[0] = (p[0] + p[1]) % m;
        for i in 1..8 {
            d[i] = (p[i - 1] + p[i] + p[i + 1]) % m;
        }
        d[8] = (p[7] + p[8]) % m;

        p = d;
    }

    let mut ans : usize = 0;

    for a in &d {
        ans += a % m;
        ans = (ans + m) % m;
    }

    println!("{}", ans);
}
