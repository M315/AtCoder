use std::io;
use std::collections::HashMap;

fn dfs(x : u64, m : u64, dp : &mut HashMap<u64,u64>) -> u64 {
    if x <= 4 {
        return x;
    }

    if dp.contains_key(&x) { 
        return dp[&x];
    }

    let mut ret : u64 = dfs(x / 2, m, dp);
    ret = (ret * dfs((x + 1) / 2, m, dp)) % m;

    dp.insert(x, ret);

    return ret;
}

fn main() {
    let mut x : String = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Filed to read line");

    let x : u64 = x.trim().parse().expect("Not a number");
    let m : u64 = 998_244_353;
    let mut dp : HashMap<u64, u64> = HashMap::new();


    println!("{}", dfs(x, m, &mut dp));
}
