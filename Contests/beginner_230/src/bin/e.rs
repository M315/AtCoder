use::proconio::input;

fn main()
{
    input!
    {
        n : u64,
    }

    let sqn : u64 = (n as f64).sqrt().floor() as u64;
    let mut last : u64 = n;
    let mut sum : u64 = n;
    
    for i in 2..=sqn
    {
        sum += n / i;
        sum += (i - 1) * (last - (n / i));
        last = n / i;
    }

    if sqn * sqn != n
    {
        if ((sqn + 1) * (sqn + 1) - sqn * sqn) / 2 <= n - sqn * sqn
        {
            sum += sqn;
        }

        if (sqn + 1) * (sqn + 1) == n + 1
        {
            sum += sqn;
        }
    }

    println!("{}", sum); 
}
