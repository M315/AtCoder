use proconio::input;
use std::collections::HashSet;

struct Move
{
    t : u64,
    a : Vec<usize>,
}

fn main()
{
    input!
    {
        n : usize,
    }

    let mut moves : Vec<Move> = Vec::new();

    for _ in 0..n
    {
        input!
        {
            t : u64,
            k : usize,
            a : [usize; k],
        }

        let m : Move = Move {
            t: t,
            a: a,
        };

        moves.push(m);
    }

    let mut time : u64 = 0;
    let mut learned : HashSet<usize> = HashSet::new();
    let mut to_learn : Vec<usize> = Vec::new();
    to_learn.push(n - 1);

    while to_learn.len() > 0
    {
        let i : usize = to_learn.pop().unwrap();

        if learned.contains(&i)
        {
            continue;
        }

        time += moves[i].t;
        learned.insert(i);

        for &m in &moves[i].a
        {
            if !learned.contains(&(m - 1))
            {
                to_learn.push(m - 1);
            }
        }
    }

    println!("{}", time);
}
