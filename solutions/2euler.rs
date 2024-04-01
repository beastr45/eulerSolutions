/*========================================
|       Simple Rust Implementation       |
========================================*/

fn main() {
    let mut x =0;
    let mut y = 1;
    let mut z =1;
    let mut sum =0;

    while x<4000000{
        x=y+z;
        if x%2 == 0 {
            sum+=x;
        }
    y=z;
    z=x;
    }

    println!("Answer: {}",sum);
}
