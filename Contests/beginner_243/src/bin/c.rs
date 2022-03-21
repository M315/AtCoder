use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

struct Pos {
    x : usize,
    y : usize,
}

fn main() {
    input! {
        n : usize
    }

    let mut p : Vec<Pos> = Vec::new();

    for _ in 0..n {
        input! {
            x : usize,
            y : usize,
        }
        
        let aux = Pos {
            x,
            y,
        };

        p.push(aux);
    }

    input! {
        s : Chars,
    }

    let mut l : HashMap<usize, i64> = HashMap::new();
    let mut r : HashMap<usize, i64> = HashMap::new();

    for i in 0..n {
        if s[i] == 'L' {
            if !l.contains_key(&p[i].y) {
                l.insert(p[i].y, p[i].x as i64);
            }
            else if l[&p[i].y] < p[i].x as i64 {
                if let Some(a) = l.get_mut(&p[i].y) {
                    *a = p[i].x as i64;
                }
            }
        }
    
        if s[i] == 'R' {
            if !r.contains_key(&p[i].y) {
                r.insert(p[i].y, p[i].x as i64);
            }
            else if r[&p[i].y] > p[i].x as i64 {
                if let Some(a) = r.get_mut(&p[i].y) {
                    *a = p[i].x as i64;
                }
            }
        }
 
        if l.contains_key(&p[i].y) && r.contains_key(&p[i].y) {
            if l[&p[i].y] >= r[&p[i].y] {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");

    return;
}
