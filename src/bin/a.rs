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
    // let source = AutoSource::from("0");
    input!{
        // from source,
        a: i64,
        b: i64,
        c: i64

    };

    if a == b {
        println!("{}", c)
    }
    else if b== c{
        println!("{}", a)

    }
    else if a == c{
        println!("{}", b)

    }
    else {
        println!("0")
    }

}
