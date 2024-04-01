fn reverse_int(mut n: i32) -> i32 {
    let mut reversed_num = 0;
    let mut remainder: i32;
    while n != 0 {
        remainder = n % 10;
        reversed_num = reversed_num * 10 + remainder;
        n /= 10;
    }
    reversed_num
}
fn check_plaindrome(n: i32) -> bool {
    if n as f32 / reverse_int(n) as f32 == 1.0 {
        println!("{}", n);
        return true;
    }
    false
}
fn check_products() -> i32 {
    let mut largest = 0;
    for i in (100..999).rev() {
        for j in (100..999).rev() {
            let product = i * j;
            if check_plaindrome(product) && product > largest {
                largest = product;
            }
        }
    }
    largest
}
fn main() {
    println!("Answer is: {}", check_products());
}
