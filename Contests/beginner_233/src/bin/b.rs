use proconio::input;

fn main()
{
    input!
    {
        l : usize,
        r : usize,
        s : String
    }

    if l > 0
    {
        print!("{}", &s[..l - 1]);
    }
    print!("{}", &s[(l - 1)..r].chars().rev().collect::<String>());
    if r < s.len()
    {
        print!("{}\n", &s[r..]);
    }
}
