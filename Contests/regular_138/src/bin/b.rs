use proconio::input;

fn main() {
    input! {
        n : usize,
        a : [usize; n]
    }

    let mut b : usize = 0;
    let mut f : bool = true;
    let mut c : i32 = 0;

    for i in 0..n {
        if !f {
            if a[i] != a[i - 1] {
                c -= 1;
            }
        } else {
            if a[i] == b {
                c += 1;
                b ^= 1;
            } else {
                f = false;
                c -= 1;
            }
        }
    }

    println!("{}", if c >= 0 { "Yes" } else { "No" });
}
