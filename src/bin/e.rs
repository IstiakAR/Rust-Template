extern crate my_lib;
use my_lib::pr;
use my_lib::{read, read_ivec, read_uvec};
// use my_lib::nd;
// use my_lib::multi_queue;
// use my_lib::algo;

fn solve() -> Option<i64> {
    let n: i32 = read!();
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..n {
        let a: i32 = read!();
        v.push(a);
    }
    let mut mx: i32 = -1;
    for i in 0..n {
        for j in 0..n {
            mx = mx.max(v[i as usize] ^ v[j as usize]);
        }
    }
    return Some(mx as i64);
}

fn main() {
    let testcases: i32 = read!();
    // let testcases = 1;
    for _ in 0..testcases {
        solve_and_print();
    }
}

#[inline]
fn solve_and_print() {
    let answer = solve();
    match answer {
        None => (),
        _ => pr::ln(answer.unwrap()),
    }
}
