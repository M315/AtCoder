use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n : usize,
    }
    let mut a : Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        input! {
            mut s : Chars,
        }
        a.push(s);
    }

    let mut c : usize;
    let mut ans : bool = false;
    for i in 0..n {
        for j in 0..n {
            if i + 5 < n {
                c = 0;
                for k in 0..6 {
                    if a[i + k][j] == '#' {
                        c += 1;
                    }
                }

                if c >= 4 {
                    ans = true;
                }
            }

            if j + 5 < n {
                c = 0;
                for k in 0..6 {
                    if a[i][j + k] == '#' {
                        c += 1;
                    }
                }

                if c >= 4 {
                    ans = true;
                }
            }

            if i + 5 < n && j + 5 < n {
                c = 0;
                for k in 0..6 {
                    if a[i + k][j + k] == '#' {
                        c += 1;
                    }
                }

                if c >= 4 {
                    ans = true;
                }
            }

            if i >= 5 && j + 5 < n {
                c = 0;
                for k in 0..6 {
                    if a[i - k][j + k] == '#' {
                        c += 1;
                    }
                }

                if c >= 4 {
                    ans = true;
                }
            }
        }
    }

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
