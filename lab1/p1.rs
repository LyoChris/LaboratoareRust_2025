//Make a function that calculates if a number is prime, and call it with every number from 0 to 100, printing the primes.

fn is_prime(x: i32) -> bool
{
    if x == 0 || x == 1 { return false; }
    for i in 2..x/2 {
        if x % i == 0 { return false; }
    }
    return true;
}

fn main() {
    for i in 0..=100 {
        match is_prime(i) {
            true => println!("{} is prime.", i),
            false => (),
        };
    }
}