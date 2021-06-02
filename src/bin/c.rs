use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    let source = AutoSource::from("2 3
    2 1
    5 10");
    input!{
        // from source,
        n: usize,
        mut k: i64,
        mut ab: [(i64, i64); n]
    };

    ab.sort_by_key(|&z| z.0);


    let mut x = 0i64;

    for &(a, b) in ab.iter(){
        if k - (a - x) < 0{
            println!("{}", x + k);
            return
        }
        k = k - (a-x) + b;
        x = a;
    }

    println!("{}", x + k)
}
