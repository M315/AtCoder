use proconio::input;
use std::collections::HashSet;

fn reduce(t : &mut (i64, i64))
{
    let d : i64 = fast_gcd(t.0, t.1);

    t.0 /= d;
    t.1 /= d;
}

// Fast iterative version of Euclid's GCD algorithm
fn fast_gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    return a;
}

fn main()
{
    input!
    {
        n : usize,
    }

    let mut cities : Vec<(i64, i64)> = Vec::new();

    for _ in 0..n
    {
        input!
        {
            x : i64,
            y : i64,
        }

        let c : (i64, i64) = (x, y); 

        cities.push(c);
    }

    let mut learned : HashSet<(i64, i64)> = HashSet::new();

    for i in 0..cities.len()
    {
        for j in i + 1..cities.len()
        {
            let mut tp : (i64, i64) = (
                cities[j].0 - cities[i].0,
                cities[j].1 - cities[i].1
            );

            reduce(&mut tp);
            learned.insert(tp);

            tp.0 = -tp.0;
            tp.1 = -tp.1;
            learned.insert(tp);
        }
    }

    println!("{}", learned.len());
}
