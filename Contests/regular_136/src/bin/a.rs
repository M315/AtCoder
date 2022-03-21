use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n : usize,
        mut s : Chars,
    }

    for i in 0..n - 1 {
        if s[i] == 'B' {
            if s[i + 1] == 'A' {
                s[i] = 'A';
                s[i + 1] = 'B';
            } else if s[i + 1] == 'B' {
                s[i] = 'A';
                s[i + 1] = ' ';
            }
        }
    }

    for c in s {
        if c != ' ' {
            print!("{}", c);
        }
    }

    println!();
}
