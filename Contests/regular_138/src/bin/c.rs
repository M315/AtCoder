use proconio::input;

fn main() {
    input! {
        n : usize,
        mut a : [i64; n],
    }

    let mut b : Vec<i64> = a.to_vec();

    a.sort();

    let mut sum : i64 = 0;
    for i in (n / 2)..n {
        sum += a[i];
    }

    let mid : i64 = a[n / 2];
    for i in 0..n {
        if b[i] < mid {
            b[i] = -1;
        } else {
            b[i] = 1;
        }
    }

    let mut k : usize = 1;
    let mut max : i64 = b[0];
    for i in 1..n {
        b[i] += b[i - 1];
        if b[i] > max {
            k = i + 1;
            max = b[i];
        }
    }
    k = (k + n) % n;

    println!("{} {}", k, sum);
}
