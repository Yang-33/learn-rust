extern crate num;

use num::Complex;


/// ここdocumentをかくところ
/// 適当にparseしてくれるのでライブラリなどで役に立つ
///
///

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}


fn main() {
    println!("Hello, world!");
    // この書き方はNodeに対応していないのでダメ
    println!("{}",escape_time(2.11,10000));
}
