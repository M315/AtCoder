use::proconio::input;

fn main()
{
    input!
    {
        n : usize,
        d : i64,
    }

    let mut wall : Vec<(i64, i64)> = Vec::new();
    for _ in 0..n
    {
        input!
        {
            l : i64,
            r : i64,
        }
        wall.push((l, r));
    }

    wall.sort_by(|a, b| a.1.cmp(&b.1));

    let mut x : i64 = std::i64::MIN;
    let mut punch : u64 = 0;

    for w in wall
    {
        // Check if the wall is already destroyed
        if x + d - 1 >= w.0
        {
            continue;
        }
        x = w.1;
        punch += 1;
    }

    println!("{}", punch);
}
