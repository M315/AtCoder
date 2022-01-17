use proconio::input;
use std::cmp;

static N : u64 = 1_000_000;

fn dfs(a : u64, n : u64, curr : u64, steps : u64, used : &mut Vec<bool>) -> u64 {
    if curr == n {
        return steps;
    }

    if curr > N || curr / 10 > n {
        return std::u64::MAX;
    }

    used[curr as usize] = true;

    // multiply by a
    let mut s1 : u64 = std::u64::MAX;
    if curr * a <= N && !used[(curr * a) as usize] {
        s1 = dfs(a, n, curr * a, steps + 1, used);
    }

    // rotate a
    let mut s2 : u64 = std::u64::MAX;

    if curr > 10 && curr % 10 != 0 {
        let mut aux : u64 = curr;
        let mut d : Vec<u64> = Vec::new();
        while aux != 0 {
            d.push(aux % 10);
            aux /= 10;
        }

        d.rotate_left(1);

        let mut m : u64 = 1;

        for i in 0..d.len() {
            aux += d[i] * m;
            m *= 10;
        }

        if !used[aux as usize] {
            s2 = dfs(a, n, aux, steps + 1, used);
        }
    }

    return cmp::min(s1, s2);
}

fn main() {
    input! {
        a : u64,
        n : u64,
    }

    let mut used : Vec<bool> = vec![false; (N + 1) as usize];
    let steps : u64 = dfs(a, n, 1, 0, &mut used);

    if steps == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", steps);
    }
}
