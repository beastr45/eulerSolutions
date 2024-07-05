fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while (i * i) <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

fn nth_prime(n: u64) -> u64 {
    let mut count = 0;
    let mut candidate = 2;
    while count < n {
        if is_prime(candidate) {
            count += 1;
        }
        if count < n {
            candidate += 1;
        }
    }
    candidate
}

fn main() {
    let n = 10001; // Change this to find a different n-th prime
    let prime = nth_prime(n);
    println!("The {}-th prime number is {}", n, prime);
}

