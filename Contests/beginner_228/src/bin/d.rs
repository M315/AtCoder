use proconio::input;

fn find(x: i64, parent : &mut Vec<i32>) -> usize
{
    if parent[x as usize] == x as i32
    {
        return x as usize;
    }
    else
    {
        parent[x as usize] = find(parent[x as usize] as i64, parent) as i32;
        return parent[x as usize] as usize;
    }
}

fn main()
{
    let n : i32 = 1 << 20;

    input!
    {
        q : usize,
    }

    let mut parent : Vec<i32> = (0..n).collect();
    let mut value : Vec<i64> = vec![-1; n as usize];

    for _ in 0..q
    {
        input!
        {
            t : u32,
            x : i64,
        }
        
        if t == 1
        {
            let i : usize = find(x % n as i64, &mut parent);
            value[i] = x;
            // As i is used set its parent to i + 1
            parent[i] = find((i + 1) as i64 % n as i64, &mut parent) as i32;

        }
        else
        {
            println!("{}", value[(x % n as i64) as usize]);
        }
    }
}
