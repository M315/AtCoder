use proconio::input;
use std::collections::HashMap;

fn main()
{
    input!
    {
        n : usize,
        names : [String; n]
    }

    let mut votes : HashMap<String, u32> = HashMap::new();
    for name in names.iter()
    {
        let k = votes.entry(name.to_string()).or_insert(0);
        *k += 1;
    }

    let mut max : (String, u32) = (String::new(), 0);
    for (name, k) in votes.iter()
    {
        if *k > max.1
        {
            max.0 = name.to_string();
            max.1 = *k;
        }
    }

    println!("{}", max.0);
}
