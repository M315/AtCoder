use proconio::input;
use proconio::marker::Bytes;

fn sum(a : &mut Vec<u8>, b : &Vec<u8>)
{
    let mut reminder : u8 = 0;
    let n = a.len();
    let m = b.len();
    let mut i = 1;

    while i <= m
    {
        a[n - i] += b[m - i] + reminder;

        reminder = 0;

        if a[n - i] >= 10
        {
            a[n - i] %= 10;
            reminder = 1;
        }

        i += 1;
    }

    while reminder == 1
    {
        if i > n
        {
            a.insert(0, 1);
            break;
        }
        else
        {
            a[n - i] += 1;
            reminder = 0;

            if a[n - i] >= 10
            {
                a[n - i] %= 10;
                reminder = 1;
            }

            i += 1;
        }
    }
}

fn main()
{
    input!
    {
        s : Bytes
    }

    let mut x : Vec<u8> = Vec::new();
    let mut ans : Vec<u8> = Vec::new();

    for c in s
    {
        let num : u8 = c - b'0';
        x.push(num);
        ans.push(num);
    }

    while !x.pop().is_none()
    {
        sum(&mut ans, &x);
    }

    for d in ans.iter()
    {
        print!("{}", d);
    }
    println!();
}
