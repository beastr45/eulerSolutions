fn main() {
    let mut number:i64 = 600851475143;
    let mut largest_prime_factor = 0;

    let mut i = 2;
    while number > 1 {
        if number % i == 0 {
            largest_prime_factor = i;
            while number % i == 0 {
                number /= i;
            }
        }
        i += 1;
    }

    println!("The largest prime factor is {}", largest_prime_factor);
}
