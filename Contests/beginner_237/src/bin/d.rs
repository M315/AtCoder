use std::io;

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n : usize = n.trim().parse().unwrap();
    
    let mut s = String::new();

    s.clear();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to readline");

    let s : &str = s.trim();

    let mut pater : Vec<usize> = (0..=n).collect();
    let mut son : Vec<usize> = (0..=n).collect();
    let mut i : usize = 1;

    for c in s.chars() {
        if c == 'R' {
            pater[i] = i - 1;

            if son[i - 1] != i - 1 {
                son[i] = son[i - 1];
                pater[son[i]] = i;
            }

            son[i - 1] = i;
        } else {
            son[i] = i - 1;
            if pater[i - 1] != i - 1 {
                pater[i] = pater[i - 1];
                son[pater[i]] = i;
            }

            pater[i - 1] = i;
        }

        i += 1;
    }
    
    // Find the first element
    let mut p : usize = 0;
    
    while pater[p] != p {
        p += 1;
    }

    while son[p] != p {
        print!("{} ", p);
        p = son[p];
    }
    println!("{}", p);
}
