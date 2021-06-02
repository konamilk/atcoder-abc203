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
//     let source = AutoSource::from("3 2
// 1 7 0
// 5 8 11
// 10 4 2
// ");
    input!{
        // from source,
        n: usize,
        k: usize,
        a: [[i64; n];n]
    };

    let mut ng = -1i64;
    let mut ok = 10_000_000_000;

    while ok - ng > 1 {
        let mid = (ok + ng) /2;
        if check(mid, n, k, &a){
            ok = mid
        }
        else {
            ng = mid
        }
    }
    println!("{}", ok)
}

fn check(x:i64, n:usize, k:usize, a:&Vec<Vec<i64>>) -> bool{
    let mut test = vec![vec![0; n]; n];
    for i in 0..n{
        for j in 0..n{
            if a[i][j] > x {
                test[i][j] = 1;
            }
        }
    }

    let cs = CumSum2D::from(&test);

    for i in 0..=n-k{
        for j in 0..=n-k{
            let total = cs.query(i, i+k,j, j+k);
            if total < k * k / 2 + 1{
                 return true
            }
        }
    }
    false
}



use cumsum2d::CumSum2D;

pub mod cumsum2d{
    use num_traits::Zero;
    use std::ops::Sub;

    #[derive(Debug, Clone)]
    pub struct CumSum2D<T>{
        pub data: Vec<Vec<T>>,
    }

    impl<T> CumSum2D<T>
    where T: Zero + Copy + Clone + Sub<Output=T>
    {
        pub fn from(in_data: &Vec<Vec<T>>)-> Self{
            let h = in_data.len();
            let w = in_data[0].len();
            let mut data = vec![vec![T::zero();w+1];h+1];
            for i in 1..=h{
                for j in 1..=w {
                    data[i][j] = data[i-1][j] + data[i][j-1] + in_data[i-1][j-1] - data[i-1][j-1];
                }
            }
            CumSum2D {data}
        }

        pub fn query(&self, i0:usize, i1:usize, j0: usize, j1:usize) -> T{
            self.data[i1][j1] - self.data[i1][j0] - self.data[i0][j1] + self.data[i0][j0]
        }
    }

    #[test]
    fn test_cumsum2d(){
        let cs =  CumSum2D::from(&vec![vec![1,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]);
        assert_eq!(cs.query(0,3, 0, 4), 78);
        assert_eq!(cs.query(0,1, 0, 1), 1);
        assert_eq!(cs.query(1,3, 1, 3), 34);

        // for i64
        let cs =  CumSum2D::from(&vec![vec![1i64,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]);
        assert_eq!(cs.query(0,3, 0, 4), 78i64);

        // for u64
        let cs =  CumSum2D::from(&vec![vec![1u64,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]);
        assert_eq!(cs.query(0,3, 0, 4), 78u64);

        // for usize
        let cs =  CumSum2D::from(&vec![vec![1usize,2,3,4], vec![5,6,7,8], vec![9,10,11,12]]);
        assert_eq!(cs.query(0,3, 0, 4), 78usize);

        // // for f64
        // // Cargo.tomlに以下を追加してね
        // // assert_approx_eq = "1.1.0"
        // use assert_approx_eq::assert_approx_eq;
        // use num_traits::Float;
        // let cs =  CumSum2D::from(&vec![vec![0.1,0.2,0.3,0.4],
        //                                vec![0.5,0.6,0.7,0.8],
        //                                vec![0.9,1.0,1.1,1.2]]);
        // assert_approx_eq!(cs.query(1, 3, 1, 3), 3.4);
    }
}
