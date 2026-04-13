extern crate my_lib;
use my_lib::{read, read_ivec, read_uvec};
use my_lib::pr;

fn solve() -> Option<String> {
    let n: usize = read!();
    let k: usize = read!();
    let mut s: String = read!();
    let mut i: i32 = 0;
    let mut v: Vec<String> = Vec::new();
    for i in 0..s.len() {
        let mut c = &s[..i+1];
        let mut p: String = String::new();
        let mut j = 0;
        j = ((k as f32) / ((i + 1) as f32)).ceil() as i32;
        while j > 0 {
            j-=1;
            p.push_str(c);
        }
        p = p[..k].to_string();
        v.push(p);
    }
    v.sort();
    return v.get(0).cloned();
}

fn main() {
    // let testcases: i32 = read!();
    let testcases = 1;
    for _ in 0..testcases { solve_and_print(); }
}

#[inline]
fn solve_and_print() {
    let answer = solve();
    match answer {
        None => (),
        _ => pr::ln(answer.unwrap())
    }
}
