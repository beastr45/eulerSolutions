fn main() {
    let mut sum=0;
    for i in 1..1000{
        if i %5==0 || i%3==0{
            sum+=i;
        }
    }
    println!("solution {}",sum);
}
