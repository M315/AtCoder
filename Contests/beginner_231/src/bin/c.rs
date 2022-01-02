use::proconio::input;

fn binary_search(a : &Vec<u64>, mut l : usize, mut r : usize, x : u64) -> usize
{
    while l <= r
    {
        let m : usize =  l + (r - l) / 2;
        if a[m] == x
        {
            return m;
        }

        if a[m] < x
        {
            l = m + 1;
        }
        else
        {
            if m == 0
            {
                break;
            }

            r = m - 1;
        }
    }

    return l;
}

fn main()
{
    input!
    {
        n : usize,
        q : usize,
        mut a : [u64; n],
        x : [u64; q],
    }

    a.sort();

    for k in x
    {
        println!("{}", a.len() - binary_search(&a, 0, a.len() - 1, k));
    }
}
