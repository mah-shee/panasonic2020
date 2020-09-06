#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        a: u128,
        b: u128,
        c: u128,
    }
    let d = c - a - b;
    if a + b < c && d.pow(2) > 4 * a * b {
        println!("Yes");
    } else {
        println!("No");
    }
}
