fn main() {
    //since 20 is a short range primes are hardcoded in and this is faster as well.
    let prime_list = [2, 3, 5, 7, 11, 13, 17, 19];
    let mut product:i64 = 1;
    for prime in prime_list.iter() {
        //not a float so this is fine
        let multiple = 20/prime;
        println!("{}",multiple*prime);
        product *= multiple*prime;
    }
    //divide by 20 because im too lazy to check for doubles
    println!("solution: {}", product/20);
}
