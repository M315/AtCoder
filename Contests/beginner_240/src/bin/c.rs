use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n : usize,
        x : usize,
    }

    let mut a : Vec<usize> = Vec::new();
    let mut b : Vec<usize> = Vec::new();

    for _ in 0..n {
        input! {
            y : usize,
            z : usize, } 
        a.push(y);
        b.push(z);
    }

    let mut lower_partial_sum : Vec<usize> = vec![0; n];
    let mut upper_partial_sum : Vec<usize> = vec![0; n];
    let mut lower : usize = 0;
    let mut upper : usize = 0;

    for i in (0..n).rev() {
        if a[i] < b[i] {
            lower += a[i];
            upper += b[i];
        } else {
            lower += b[i];
            upper += a[i];
        }
        lower_partial_sum[i] = lower;
        upper_partial_sum[i] = upper;
    }

    let mut dp : HashSet<usize> = HashSet::new();

    dp.insert(a[0]);
    dp.insert(b[0]);

    for i in 1..n {
        let mut aux : HashSet<usize> = HashSet::new();
        for p in dp.drain() {
            if p + upper_partial_sum[i] >= x && p + lower_partial_sum[i] <= x {
                aux.insert(p + a[i]);
                aux.insert(p + b[i]);
            }
        }

        dp = aux;
        
        if dp.is_empty() {
            break;
        }
    }

    if dp.get(&x) != None {
        println!("Yes");
    } else {
        println!("No");
    }
}
