use proconio::input;

static MOD : u64 = 998_244_353;

fn triangular(n : u64) -> u64 {
    let mut ret : u64 = n % MOD;

    ret = ((ret * (ret + 1)) / 2) % MOD;

    return ret;
}

fn main() {
    input! {
        n : u64,
    }

    let mut d : u64 = 0;
    let mut a : u64 = n;

    while a != 0 {
        d += 1;
        a /= 10;
    }

    let mut ans : u64 = 0;
    a = 1;

    for _ in 1..d {
        ans += triangular(9 * a);
        ans %= MOD;
        a *= 10;
    }

    ans += triangular(n + 1 - a);
    ans %= MOD;

    println!("{}", ans);
}
