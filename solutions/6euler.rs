fn main() {
    let n = 100;
    let sum = (n*(n+1))/2;
    let sum_pow = sum*sum;
    let mut powers_sum = 0;
    for i in 1..=n{
        powers_sum+=i*i;
    }
    let difference = sum_pow-powers_sum;
    println!("Answer: {}",difference);
}
