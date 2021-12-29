use proconio::input;

fn dfs(pos : usize, seki : u64, x : u64, n : usize, a : &Vec<Vec<u64>>, ans : &mut u64)
{
    if pos == n
    {
        if seki == x
        {
            *ans += 1
        }
        return;
    }
    for k in a[pos].iter()
    {
        if seki <= x / k
        {
            dfs(pos + 1, seki * k, x, n, a, ans);
        }
    }
}

fn main()
{
    input!
    {
        n : usize,
        x : u64,
    }

    let mut a : Vec<Vec<u64>> = Vec::new();
    for _ in 0..n
    {
        input!
        {
            m : usize,
            aux : [u64; m]
        }
        a.push(aux);
    }

    let mut ans : u64 = 0;
    dfs(0, 1, x, n, &a, &mut ans);
    println!("{}", ans);
}
