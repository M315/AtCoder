use proconio::input;
use proconio::marker::Chars;
use std::cmp;

fn main()
{
    input!
    {
        h : usize,
        w : usize,
        grid : [Chars; h]
    }

    let mut dist : Vec<Vec<u64>> = vec![vec![0; w + 1]; h + 1];

    for i in (0..h).rev()
    {
        for j in (0..w).rev()
        {
            if grid[i][j] == '.'
            {
                dist[i][j] = cmp::max(dist[i + 1][j], dist[i][j + 1]) + 1;
            }
        }
    }

    println!("{}", dist[0][0]);
}
